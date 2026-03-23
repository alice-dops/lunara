use std::sync::Mutex;

use tauri::{AppHandle, State};

use crate::{
    manger::{app_manager::AppManager, snippets_manager::SnippetsManager},
    models::{app_entry::AppView, snipsets::SnippetView, system_status::SystemStatus},
    system::{collect::collect_system_status, window_manager_controller::WMState},
};

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
pub fn list_snippets(state: State<'_, Mutex<SnippetsManager>>) -> Vec<SnippetView> {
    let mgr = state.lock().unwrap();
    mgr.get_snippet_views()
}

#[tauri::command]
pub fn run_app(mgr: State<'_, Mutex<AppManager>>, wm_state: State<'_, WMState>, id: String) {
    let app = {
        let mgr = mgr.lock().unwrap();
        mgr.find_app(&id).cloned() // or clone whatever you need
    };

    if let Some(app) = app {
        let _ = wm_state.with_wm(|wm| wm.launch_or_focus(&app));
    }
}

#[tauri::command]
pub fn run_snippet(
    app: AppHandle,
    state: State<'_, Mutex<SnippetsManager>>,
    id: String,
    button: String,
) {
    let mut mgr = state.lock().unwrap();
    let _ = mgr.run_snipsset(&app, id, button);
}

#[tauri::command]
pub fn get_system_status() -> SystemStatus {
    collect_system_status()
}
