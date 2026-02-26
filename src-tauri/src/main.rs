// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Manager, State};

use crate::manger::{
    app_manager::AppManager, gamepad_manager::start_gilrs_forwarder,
    snippets_manager::SnippetsManager,
};

mod commands;
mod config;
mod manger;
mod models;
mod system;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppManager::new()))
        .manage(Mutex::new(SnippetsManager::new()))
        .setup(|app| {
            let handle = app.handle();
            {
                start_gilrs_forwarder(handle.clone());
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
