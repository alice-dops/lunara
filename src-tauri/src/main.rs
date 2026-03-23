// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Manager, State};

use crate::{
    config::loader::load_config,
    manger::{
        app_manager::AppManager,
        gamepad_manager::start_gilrs_forwarder,
        snippets_manager::{start_snippets_monitor, SnippetsManager},
    },
    system::window_manager_controller::{get_wm_from_name, WMState},
};

mod commands;
mod config;
mod manger;
mod models;
mod system;

fn main() {
    let conf = load_config();
    let wm = get_wm_from_name(&conf.window_manager);
    if let Err(e) = &wm {
        eprintln!("{e}")
    }

    let wmb = wm.unwrap();

    tauri::Builder::default()
        .manage(Mutex::new(conf.clone()))
        .manage(Mutex::new(AppManager::new()))
        .manage(Mutex::new(SnippetsManager::new()))
        .manage(WMState {
            wm: Mutex::new(wmb),
        })
        .setup(|app| {
            // let config: tauri::State<'_, Mutex<LunaraConfig>> = app.state();
            // let conf = config.lock().unwrap();

            let handle = app.handle();
            {
                start_gilrs_forwarder(handle.clone(), conf);
            }
            {
                let state: State<'_, Mutex<AppManager>> = app.state();
                let mut mgr = state.lock().unwrap();
                mgr.reload_config();
            }
            {
                let state: State<'_, Mutex<SnippetsManager>> = app.state();
                let mut mgr = state.lock().unwrap();
                mgr.reload_config();
            }
            start_snippets_monitor(app.handle().clone());
            {
                system::monitor::start_system_monitor(handle.clone());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::apps::list_apps,
            commands::apps::list_snippets,
            commands::apps::run_app,
            commands::apps::run_snippet,
            commands::apps::get_system_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
