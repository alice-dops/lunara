use std::sync::Mutex;

use tauri::State;

use crate::{
    manger::{app_manager::AppManager, snippets_manager::SnippetsManager},
    models::{app_entry::AppEntry, snipsets::Snippet, system_status::SystemStatus},
    system::collect::collect_system_status,
};

#[derive(serde::Serialize)]
pub struct AppView {
    pub app: AppEntry,
}

#[tauri::command]
pub fn list_apps(state: State<'_, Mutex<AppManager>>) -> Vec<AppView> {
    let mgr = state.lock().unwrap();
    mgr.apps()
        .iter()
        .cloned()
        .map(|app| AppView { app })
        .collect()
}

#[tauri::command]
pub fn list_snippets(state: State<'_, Mutex<SnippetsManager>>) -> Vec<Snippet> {
    let mgr = state.lock().unwrap();
    mgr.snippets().iter().cloned().collect()
}

#[tauri::command]
pub fn run_app(state: State<'_, Mutex<AppManager>>, id: String) {
    let mgr = state.lock().unwrap();
    let _ = mgr.run_app(id);
}

#[tauri::command]
pub fn run_snippet(state: State<'_, Mutex<SnippetsManager>>, id: String, button: String) {
    let mgr = state.lock().unwrap();
    let _ = mgr.run_snipsset(id, button);
}

#[tauri::command]
pub fn get_system_status() -> SystemStatus {
    return collect_system_status();
}
