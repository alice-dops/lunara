use gilrs::{ev::Code, Axis, Button, EventType, Gilrs};
use std::{
    thread,
    time::{Duration, SystemTime},
};
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

use crate::system::bspwm::focus_lunara;

const DEADZONE: f32 = 0.35;
const INIT_DELAY_MS: u128 = 250;
const REPEAT_DELAY_MS: u128 = 110;

#[derive(serde::Serialize, Clone)]
#[serde(tag = "type", content = "value")]
enum InputEvent {
    Dir(String),
    Btn(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
    None = 0,
}

fn dominant_dir(x: f32, y: f32, deadzone: f32) -> Direction {
    let ax = x.abs();
    let ay = y.abs();

    if ax < deadzone && ay < deadzone {
        Direction::None
    } else if ax > ay {
        if x > 0.0 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else {
        if y > 0.0 {
            Direction::Down
        } else {
            Direction::Up
        }
    }
}

impl Direction {
    pub fn to_string(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::None => "none",
        }
    }
}

pub struct GamepadManager {
    app: AppHandle,
    stick_x: f32,
    stick_y: f32,
    stick_dir: Direction,
    stick_next_time: u128,
    dpad_dir: Direction,
    dpad_next_time: u128,
    window: WebviewWindow,
}

pub fn start_gilrs_forwarder(app: AppHandle) {
    thread::spawn(move || {
        let mut manager = GamepadManager::new(app);
        let mut gilrs = Gilrs::new().expect("gilrs init");

        loop {
            let mut got_any = false;
            while let Some(ev) = gilrs.next_event() {
                got_any = true;
                match ev.event {
                    EventType::ButtonPressed(btn, code) => {
                        manager.button_pressed(ev.time, btn, code);
                    }
                    EventType::ButtonReleased(btn, code) => {
                        manager.button_releassed(ev.time, btn, code);
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        manager.axis_change(ev.time, axis, value)
                    }
                    _ => {}
                }
            }
            if !got_any {
                thread::sleep(Duration::from_millis(15));
            }
        }
    });
}

impl GamepadManager {
    pub fn new(app: AppHandle) -> Self {
        let win = app.get_webview_window("main").unwrap();
        Self {
            app,
            stick_x: 0.,
            stick_y: 0.,
            stick_dir: Direction::None,
            stick_next_time: 0,
            dpad_dir: Direction::None,
            dpad_next_time: 0,
            window: win,
        }
    }

    fn home_button_pressed(&self) {
        let _ = focus_lunara();
    }

    pub fn button_pressed(&mut self, time: SystemTime, btn: Button, _: Code) {
        if btn == Button::Mode {
            self.home_button_pressed();
            return;
        }
        if !self.window.is_focused().unwrap_or(false) {
            return;
        }
        if btn.is_dpad() {
            self.dpad_event(time, btn, true);
        }
    }

    pub fn button_releassed(&mut self, time: SystemTime, btn: Button, _: Code) {
        if !self.window.is_focused().unwrap_or(false) {
            return;
        }
        if btn.is_action() {
            let label = match btn {
                Button::North => "Y",
                Button::South => "A",
                Button::East => "B",
                Button::West => "X",
                Button::C => "C",
                Button::Z => "Z",
                _ => return,
            };
            self.send_event_acction_press(label);
            return;
        }
        if btn.is_dpad() {
            self.dpad_event(time, btn, false);
        }
    }

    pub fn axis_change(&mut self, time: SystemTime, axis: Axis, value: f32) {
        if !self.window.is_focused().unwrap_or(false) {
            return;
        }
        if axis == Axis::LeftStickX {
            self.stick_x = value
        } else if axis == Axis::LeftStickY {
            self.stick_y = value
        } else {
            return;
        }

        let dir = dominant_dir(self.stick_x, self.stick_y, DEADZONE);

        let t = time.elapsed().unwrap().as_millis();

        if dir == Direction::None {
            self.stick_next_time = 0;
        } else if dir != self.stick_dir {
            self.stick_next_time = t + INIT_DELAY_MS;
            self.send_event_direction(&dir);
        } else if t >= self.stick_next_time {
            self.stick_next_time = t + REPEAT_DELAY_MS;
            self.send_event_direction(&dir);
        }

        self.stick_dir = dir
    }

    fn dpad_event(&mut self, time: SystemTime, btn: Button, press: bool) {
        let dir = match btn {
            Button::DPadUp => Direction::Up,
            Button::DPadDown => Direction::Down,
            Button::DPadLeft => Direction::Left,
            Button::DPadRight => Direction::Right,
            _ => return,
        };

        if press {
            if self.dpad_dir != dir {
                self.dpad_dir = dir;
                self.dpad_next_time = 0;
            }
        } else {
            if self.dpad_dir == dir {
                self.dpad_dir = Direction::None;
                self.dpad_next_time = 0;
            }
            return;
        }

        if self.dpad_dir != Direction::None {
            let t = time.elapsed().unwrap().as_millis();
            if self.dpad_next_time == 0 {
                self.dpad_next_time = t + INIT_DELAY_MS;
                self.send_event_direction(&self.dpad_dir);
            } else if t >= self.dpad_next_time {
                self.dpad_next_time = t + REPEAT_DELAY_MS;
                self.send_event_direction(&self.dpad_dir);
            }
        }
    }

    fn send_event_acction_press(&self, name: &str) {
        let _ = self
            .app
            .emit("lunara://input", InputEvent::Btn(name.to_string()));
    }

    fn send_event_direction(&self, dir: &Direction) {
        let _ = self.app.emit(
            "lunara://input",
            InputEvent::Dir(dir.to_string().to_string()),
        );
    }
}
