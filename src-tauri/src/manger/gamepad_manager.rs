use enigo::{Enigo, Key, Keyboard, Mouse, Settings};
use gilrs::{ev::Code, Axis, Button, EventType, Gilrs};
use std::{
    collections::HashMap,
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
const LUNARA_CONFIG_FOCUS_GLOBAL: &str = "global";
const LUNARA_CONFIG_FOCUS_LUNARA: &str = "lunara";

const MOUSE_SPEED: f32 = 16.;

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
    config: LunaraConfig,
    global_keymap: LunaraConfigGamepadKeymapEntry,
    lunara_keymap: Option<LunaraConfigGamepadKeymapEntry>,
    current_focus: String,
    current_focus_keymap: Option<LunaraConfigGamepadKeymapEntry>,
    engine: Enigo,
    stick_x: f32,
    stick_y: f32,
    stick_dir: Direction,
    stick_next_time: u128,
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
        //         println!("Can't open X11 connection. If your desktop doesn't run X11, rebuild this binary without x11 features: {err:?}");
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
                    EventType::ButtonRepeated(btn, code) => {
                        manager.button_pressed_releassed(false, ev.time, btn, code);
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
            manager.axis_update(time);
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
        let mut d_map: HashMap<Button, LunaraConfigGamapedKeyEntry> = HashMap::new();
        d_map.insert(
            Button::Mode,
            LunaraConfigGamapedKeyEntry {
                home: true,
                mouse: None,
                keyboard: None,
                cmd: None,
                args: None,
            },
        );
        let default_global = LunaraConfigGamepadKeymapEntry {
            stick_mode: LunaraConfigStickMode::None,
            keys: d_map,
        };
        let global_keymap = config
            .gamepad_keymap
            .get(&LUNARA_CONFIG_FOCUS_GLOBAL.to_string())
            .unwrap_or(&default_global)
            .clone();
        Self {
            app,
            config: config.clone(),
            global_keymap: global_keymap,
            current_focus: "".to_string(),
            lunara_keymap: config
                .gamepad_keymap
                .get(&LUNARA_CONFIG_FOCUS_LUNARA.to_string())
                .cloned(),
            current_focus_keymap: None,
            engine: Enigo::new(&Settings::default()).unwrap(),
            stick_x: 0.,
            stick_y: 0.,
            stick_dir: Direction::None,
            stick_next_time: 0,
            dpad_dir: Direction::None,
            dpad_next_time: 0,
            window: win,
        }
    }

    fn refresh_current_focused(&mut self) {
        let state: tauri::State<'_, WMState> = self.app.state();
        if let Ok(v) = state.with_wm(|wm| wm.get_current_focused()) {
            self.current_focus_keymap = self.config.gamepad_keymap.get(&v).cloned();
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
        if let Some(modifiers) = &keyboard.modifer {
            for &modifier in modifiers {
                let _ = self.engine.key(modifier, enigo::Direction::Press);
            }
        }

        for &ch in &keyboard.keys {
            let _ = self
                .engine
                .key(enigo::Key::Unicode(ch), enigo::Direction::Click);
        }

        if let Some(modifiers) = &keyboard.modifer {
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
        // global
        if let Some(k) = self.global_keymap.keys.get(&btn) {
            self.exec_lunara_gampad_key(press, k.clone());
        }

        // lunara
        if self.window.is_focused().unwrap_or(false) {
            if btn.is_dpad() {
                self.dpad_event(time, btn, press);
                return;
            }

            if btn.is_action() {
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

            if let Some(km) = &self.lunara_keymap {
                if let Some(k) = km.keys.get(&btn) {
                    self.exec_lunara_gampad_key(press, k.clone());
                }
            }

            return;
        }

        // app
        if let Some(km) = &self.current_focus_keymap {
            if let Some(k) = km.keys.get(&btn) {
                self.exec_lunara_gampad_key(press, k.clone());
            }
        }
    }

    pub fn axis_change(&mut self, _time: SystemTime, axis: Axis, value: f32) {
        if axis == Axis::LeftStickX {
            self.stick_x = value
        } else if axis == Axis::LeftStickY {
            self.stick_y = value
        } else {
            return;
        }
    }

    pub fn axis_update(&mut self, time: SystemTime) {
        let dir = dominant_dir(self.stick_x, self.stick_y, DEADZONE);

        let t = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut dir_this_tick = Direction::None;
        if dir == Direction::None {
            self.stick_next_time = 0;
        } else if dir != self.stick_dir {
            self.stick_next_time = t + INIT_DELAY_MS;
            dir_this_tick = dir
        } else if t >= self.stick_next_time {
            self.stick_next_time = t + REPEAT_DELAY_MS;
            dir_this_tick = dir
        }

        self.stick_dir = dir;

        if self.window.is_focused().unwrap_or(false) {
            if dir != Direction::None {
                self.send_event_direction(&dir_this_tick);
            }
            return;
        }
        if let Some(km) = &self.current_focus_keymap {
            if km.stick_mode == LunaraConfigStickMode::None {
                let _ = self.engine.move_mouse(
                    (self.stick_x * MOUSE_SPEED) as i32,
                    (self.stick_y * MOUSE_SPEED * -1.) as i32,
                    enigo::Coordinate::Rel,
                );
            } else if km.stick_mode == LunaraConfigStickMode::Arrow {
                let _ = match dir_this_tick {
                    Direction::Up => self.engine.key(Key::UpArrow, enigo::Direction::Click),
                    Direction::Down => self.engine.key(Key::DownArrow, enigo::Direction::Click),
                    Direction::Left => self.engine.key(Key::LeftArrow, enigo::Direction::Click),
                    Direction::Right => self.engine.key(Key::RightArrow, enigo::Direction::Click),
                    Direction::None => Ok(()),
                };
            }
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
