use std::{collections::HashMap, env};

use enigo::Key;
use gilrs::Button;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct LunaraConfig {
    pub window_manager: String,
    pub user_name: String,
    pub gamepad_keymap: HashMap<String, LunaraConfigGamepadKeymapEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct LunaraConfigGamepadKeymapEntry {
    pub stick_mode: LunaraConfigStickMode,
    pub keys: HashMap<Button, LunaraConfigGamapedKeyEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct LunaraConfigGamapedKeyEntry {
    pub home: bool,
    pub mouse: Option<enigo::Button>,
    pub keyboard: Option<LunaraConfigKeyboard>,
    pub cmd: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LunaraConfigKeyboard {
    pub modifer: Option<Vec<Key>>,
    pub keys: Vec<char>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LunaraConfigStickMode {
    #[serde(alias = "mouse")]
    Mouse,
    #[serde(alias = "arrow")]
    Arrow,
    #[serde(alias = "none")]
    #[default]
    None,
}

impl Default for LunaraConfig {
    fn default() -> Self {
        let wm = env::var("XDG_CURRENT_DESKTOP")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "bspwm".to_string());

        LunaraConfig {
            window_manager: wm,
            user_name: "Alice".to_string(),
            gamepad_keymap: HashMap::new(),
        }
    }
}

impl Default for LunaraConfigGamepadKeymapEntry {
    fn default() -> Self {
        LunaraConfigGamepadKeymapEntry {
            stick_mode: LunaraConfigStickMode::None,
            keys: HashMap::new(),
        }
    }
}

impl Default for LunaraConfigGamapedKeyEntry {
    fn default() -> Self {
        LunaraConfigGamapedKeyEntry {
            home: false,
            mouse: None,
            keyboard: None,
            cmd: None,
            args: None,
        }
    }
}
