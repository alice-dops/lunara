use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LunaraConfig {
    pub window_manager: String,
    pub user_name: String,
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
        }
    }
}
