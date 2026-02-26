use crate::models::system_status::SystemStatus;
use std::{fs, process::Command};

fn run(cmd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn get_volume() -> (u8, bool) {
    let s = run("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"]).unwrap_or_default();
    let muted = s.to_uppercase().contains("MUTED");

    // Find first float like 0.45
    let vol = s
        .split_whitespace()
        .find_map(|tok| tok.parse::<f32>().ok())
        .unwrap_or(0.0);

    let pct = (vol * 100.0).round().clamp(0.0, 100.0) as u8;
    (pct, muted)
}

// Battery via sysfs: /sys/class/power_supply/BAT*/capacity + status
pub fn get_battery() -> (Option<u8>, Option<bool>) {
    let base = "/sys/class/power_supply";
    let entries = fs::read_dir(base).ok();

    let Some(entries) = entries else {
        return (None, None);
    };

    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        if !name.starts_with("BAT") {
            continue;
        }

        let cap_path = format!("{}/{}/capacity", base, name);
        let status_path = format!("{}/{}/status", base, name);

        let cap = fs::read_to_string(&cap_path)
            .ok()
            .and_then(|s| s.trim().parse::<u8>().ok());

        let charging = fs::read_to_string(&status_path).ok().map(|s| {
            let t = s.trim().to_lowercase();
            t == "charging" || t == "full"
        });

        return (cap, charging);
    }

    (None, None)
}

pub fn get_wifi() -> (bool, Option<String>) {
    let wifi_enabled = run("nmcli", &["-t", "-f", "WIFI", "g"])
        .map(|s| s == "enabled")
        .unwrap_or(false);

    if !wifi_enabled {
        return (false, None);
    }

    let list = run("nmcli", &["-t", "-f", "ACTIVE,SSID", "dev", "wifi"]).unwrap_or_default();
    for line in list.lines() {
        if let Some(rest) = line.strip_prefix("yes:") {
            let ssid = rest.trim().to_string();
            return (true, if ssid.is_empty() { None } else { Some(ssid) });
        }
    }

    (true, None)
}

pub fn collect_system_status() -> SystemStatus {
    let (volume_percent, muted) = get_volume();
    let (battery_percent, charging) = get_battery();
    let (wifi_up, ssid) = get_wifi();

    SystemStatus {
        volume_percent,
        muted,
        battery_percent,
        charging,
        wifi_up,
        ssid,
    }
}
