use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct SystemStatus {
    pub volume_percent: u8, // 0-100
    pub muted: bool,
    pub battery_percent: Option<u8>, // None if no battery
    pub charging: Option<bool>,
    pub wifi_up: bool,
    pub ssid: Option<String>,
}
