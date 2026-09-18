use enigo::{Enigo, Key, Keyboard, Mouse, Settings};
use gilrs::{ev::Code, Axis, Button, EventType, Gilrs};
use std::{
    thread,
    time::{Duration, SystemTime},
};
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

use crate::{
    models::lunara_config::{
        LunaraConfig, LunaraConfigGamapedKeyEntry, LunaraConfigGamepadKeymapEntry,
        LunaraConfigKeyboard, LunaraConfigStickMode,
    },
    system::{command::run_optional, window_manager_controller::WMState},
};

const DEADZONE: f32 = 0.35;
const INIT_DELAY_MS: u128 = 250;
const REPEAT_DELAY_MS: u128 = 200;

const MOUSE_SPEED: f32 = 16.;
const SCROLL_SPEED: f32 = 4.;

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

#[derive(Clone)]
pub struct StickState {
    x: f32,
    y: f32,
    dir: Direction,
    next_time: u128,
}

impl StickState {
    fn stick_update(&mut self, time: SystemTime) -> Direction {
        let dir = dominant_dir(self.x, self.y, DEADZONE);
        let t = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut dir_this_tick = Direction::None;
        if dir == Direction::None {
            self.next_time = 0;
        } else if dir != self.dir {
            self.next_time = t + INIT_DELAY_MS;
            dir_this_tick = dir;
        } else if t >= self.next_time {
            self.next_time = t + REPEAT_DELAY_MS;
            dir_this_tick = dir;
        }

        self.dir = dir;

        dir_this_tick
    }
}

impl Default for StickState {
    fn default() -> Self {
        StickState {
            x: 0.,
            y: 0.,
            dir: Direction::None,
            next_time: 0,
        }
    }
}

pub struct GamepadManager {
    app: AppHandle,
    config: LunaraConfig,
    current_focus: String,
    current_focus_keymap: Option<LunaraConfigGamepadKeymapEntry>,
    engine: Enigo,
    stick_left: StickState,
    stick_right: StickState,
    dpad_dir: Direction,
    dpad_next_time: u128,
    window: WebviewWindow,
}

pub fn start_gilrs_forwarder(app: AppHandle, config: LunaraConfig) {
    thread::spawn(move || {
        let mut manager = GamepadManager::new(app, config);
        let mut gilrs = Gilrs::new().expect("gilrs init");
        let mut counter = 0u32;

        // #[cfg(feature = "x11rb")]
        // let (x11, _) = match x11rb::connect(None) {
        //     Ok(conn) => conn,
        //     Err(err) => {
        // println!("Can't open X11 connection. If your desktop doesn't run X11, rebuild this binary without x11 features: {err:?}");
        //         return;
        //     }
        // };

        loop {
            counter += 1;
            while let Some(ev) = gilrs.next_event() {
                match ev.event {
                    EventType::ButtonPressed(btn, code) => {
                        manager.button_pressed_releassed(true, ev.time, btn, code);
                    }
                    EventType::ButtonReleased(btn, code) => {
                        manager.button_pressed_releassed(false, ev.time, btn, code);
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        manager.axis_change(ev.time, axis, value);
                    }
                    _ => {}
                }
            }
            let time = SystemTime::now();
            manager.axis_update_all(time);
            // ~1s
            if counter >= 60 {
                manager.refresh_current_focused();
                counter = 0;
            }
            thread::sleep(Duration::from_millis(16));
        }
    });
}

impl GamepadManager {
    pub fn new(app: AppHandle, config: LunaraConfig) -> Self {
        println!("{:?}", config);
        let win = app.get_webview_window("main").unwrap();
        Self {
            app,
            config: config.clone(),
            current_focus: "".to_string(),
            current_focus_keymap: None,
            engine: Enigo::new(&Settings::default()).unwrap(),
            stick_left: StickState::default(),
            stick_right: StickState::default(),
            dpad_dir: Direction::None,
            dpad_next_time: 0,
            window: win,
        }
    }

    fn refresh_current_focused(&mut self) {
        let state: tauri::State<'_, WMState> = self.app.state();
        if let Ok(v) = state.with_wm(|wm| wm.get_current_focused()) {
            self.current_focus_keymap = self.config.desktop_keymap.get(&v).cloned();
            self.current_focus = v;
        }
    }

    fn home_button_pressed(&self) {
        let state: tauri::State<'_, WMState> = self.app.state();
        if self.window.is_focused().unwrap_or(false) {
            let _ = state.with_wm(|wm| wm.focus_last_app());
        } else {
            let _ = state.with_wm(|wm| wm.focus_lunara());
        };
    }

    fn exec_lunara_gampad_key(&mut self, press: bool, key: LunaraConfigGamapedKeyEntry) {
        if press {
            return;
        }

        if key.home {
            self.home_button_pressed();
            return;
        }
        if let Some(cmd) = &key.cmd {
            let args = key.args.as_deref().unwrap_or(&[]);
            let _ = run_optional(cmd, args);
        }

        if let Some(b) = &key.keyboard.clone() {
            self.exec_keyboard_binding(b);
        }

        if let Some(b) = key.mouse {
            let _ = self.engine.button(b, enigo::Direction::Click);
        }
    }

    fn exec_keyboard_binding(&mut self, keyboard: &LunaraConfigKeyboard) {
        if let Some(modifiers) = &keyboard.modifier {
            for &modifier in modifiers {
                let _ = self.engine.key(modifier, enigo::Direction::Press);
            }
        }

        for &ch in &keyboard.keys {
            let _ = self
                .engine
                .key(enigo::Key::Unicode(ch), enigo::Direction::Click);
        }

        if let Some(modifiers) = &keyboard.modifier {
            for &modifier in modifiers.iter().rev() {
                let _ = self.engine.key(modifier, enigo::Direction::Release);
            }
        }
    }

    pub fn button_pressed_releassed(
        &mut self,
        press: bool,
        time: SystemTime,
        btn: Button,
        _: Code,
    ) {
        // app
        if let Some(km) = &self.current_focus_keymap {
            if let Some(k) = km.keys.get(&btn) {
                self.exec_lunara_gampad_key(press, k.clone());
                return;
            }
        }

        // global
        if let Some(k) = self.config.global_keymap.keys.get(&btn) {
            self.exec_lunara_gampad_key(press, k.clone());
        }

        // lunara
        if self.window.is_focused().unwrap_or(false) {
            if btn.is_dpad() {
                self.dpad_event(time, btn, press);
                return;
            }

            if btn.is_action() {
                if press {
                    return;
                }
                if let Some(label) = match btn {
                    Button::North => Some("Y"),
                    Button::South => Some("A"),
                    Button::East => Some("B"),
                    Button::West => Some("X"),
                    Button::C => Some("C"),
                    Button::Z => Some("Z"),
                    _ => None,
                } {
                    self.send_event_acction_press(label);
                    return;
                }
            }

            if let Some(k) = self.config.lunara_keymap.keys.get(&btn) {
                self.exec_lunara_gampad_key(press, k.clone());
            }

            return;
        }
    }

    pub fn axis_change(&mut self, _time: SystemTime, axis: Axis, value: f32) {
        if axis == Axis::LeftStickX {
            self.stick_left.x = value
        } else if axis == Axis::LeftStickY {
            self.stick_left.y = value
        } else if axis == Axis::RightStickX {
            self.stick_right.x = value
        } else if axis == Axis::RightStickY {
            self.stick_right.y = value
        } else {
            return;
        }
    }

    fn stick_process_mode(
        &mut self,
        stick: StickState,
        dir: Direction,
        mode: LunaraConfigStickMode,
    ) -> bool {
        if mode == LunaraConfigStickMode::Mouse {
            let _ = self.engine.move_mouse(
                (stick.x * MOUSE_SPEED) as i32,
                (stick.y * MOUSE_SPEED * -1.) as i32,
                enigo::Coordinate::Rel,
            );
            return true;
        }

        if mode == LunaraConfigStickMode::Arrow {
            let _ = match dir {
                Direction::Up => self.engine.key(Key::UpArrow, enigo::Direction::Click),
                Direction::Down => self.engine.key(Key::DownArrow, enigo::Direction::Click),
                Direction::Left => self.engine.key(Key::LeftArrow, enigo::Direction::Click),
                Direction::Right => self.engine.key(Key::RightArrow, enigo::Direction::Click),
                Direction::None => Ok(()),
            };
            return true;
        }

        if mode == LunaraConfigStickMode::Scroll {
            let _ = self
                .engine
                .scroll((stick.x * SCROLL_SPEED) as i32, enigo::Axis::Horizontal);

            let _ = self
                .engine
                .scroll((stick.y * SCROLL_SPEED * -1.) as i32, enigo::Axis::Vertical);
            return true;
        }

        return false;
    }

    pub fn axis_update_all(&mut self, time: SystemTime) {
        let right_dir = self.stick_right.stick_update(time);
        let left_dir = self.stick_left.stick_update(time);

        // lunara
        if self.window.is_focused().unwrap_or(false) {
            if self.stick_left.dir != Direction::None {
                self.send_event_direction(&left_dir);
            }
            return;
        }

        let mut skip_right = false;
        let mut skip_left = false;

        // app
        if let Some(km) = &self.current_focus_keymap.clone() {
            skip_right = self.stick_process_mode(
                self.stick_right.clone(),
                right_dir,
                km.right_stick_mode.clone(),
            );
            skip_left = self.stick_process_mode(
                self.stick_left.clone(),
                left_dir,
                km.left_stick_mode.clone(),
            );
        }

        // global
        if !skip_right {
            self.stick_process_mode(
                self.stick_right.clone(),
                right_dir,
                self.config.global_keymap.right_stick_mode.clone(),
            );
        }
        if !skip_left {
            self.stick_process_mode(
                self.stick_left.clone(),
                left_dir,
                self.config.global_keymap.left_stick_mode.clone(),
            );
        }
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
