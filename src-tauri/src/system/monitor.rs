use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use tokio::time::{interval, Duration};

use crate::models::system_status::SystemStatus;
use crate::system::collect;

fn emit_snapshot(app: &AppHandle, snap: &SystemStatus) {
    let _ = app.emit("lunara://system", snap.clone());
}

pub fn start_system_monitor(app: AppHandle) {
    let shared = Arc::new(Mutex::new(SystemStatus::default()));

    {
        let app = app.clone();
        let shared = Arc::clone(&shared);

        tauri::async_runtime::spawn(async move {
            let mut tick = interval(Duration::from_millis(250));

            loop {
                tick.tick().await;
                let (vol, muted) = collect::get_volume();

                let mut snap = shared.lock().unwrap();
                let mut changed = false;

                if snap.volume_percent != vol {
                    snap.volume_percent = vol;
                    changed = true;
                }
                if snap.muted != muted {
                    snap.muted = muted;
                    changed = true;
                }

                if changed {
                    emit_snapshot(&app, &snap);
                }
            }
        });
    }

    {
        let app = app.clone();
        let shared = Arc::clone(&shared);

        tauri::async_runtime::spawn(async move {
            let mut tick = interval(Duration::from_secs(3));

            loop {
                tick.tick().await;
                let (wifi_up, ssid) = collect::get_wifi();

                let mut snap = shared.lock().unwrap();
                let mut changed = false;

                if snap.wifi_up != wifi_up {
                    snap.wifi_up = wifi_up;
                    changed = true;
                }
                if snap.ssid != ssid {
                    snap.ssid = ssid;
                    changed = true;
                }

                if changed {
                    emit_snapshot(&app, &snap);
                }
            }
        });
    }

    {
        let app = app.clone();
        let shared = Arc::clone(&shared);

        tauri::async_runtime::spawn(async move {
            let mut tick = interval(Duration::from_secs(15));

            loop {
                tick.tick().await;
                let (battery_percent, charging) = collect::get_battery();

                let mut snap = shared.lock().unwrap();
                let mut changed = false;

                if snap.battery_percent != battery_percent {
                    snap.battery_percent = battery_percent;
                    changed = true;
                }
                if snap.charging != charging {
                    snap.charging = charging;
                    changed = true;
                }

                if changed {
                    emit_snapshot(&app, &snap);
                }
            }
        });
    }
}
