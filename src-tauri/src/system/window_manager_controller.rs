use std::sync::Mutex;

use crate::{
    models::app_entry::AppEntry,
    system::{bspwm_wm_controller::BspwmWMController, command::LunaraResult},
};

pub struct WMState {
    // pub wm: Mutex<Option<Box<dyn WindowManagerController + Send>>>,
    pub wm: Mutex<Box<dyn WindowManagerController + Send>>,
}

impl WMState {
    pub fn with_wm<F, R>(&self, f: F) -> LunaraResult<R>
    where
        F: FnOnce(&mut dyn WindowManagerController) -> LunaraResult<R>,
    {
        let mut guard = self.wm.lock().unwrap();
        let wm = guard.as_mut();
        f(wm)
    }
}

pub trait WindowManagerController: Send {
    fn new() -> Self
    where
        Self: Sized;

    fn is_running(&mut self, app: &AppEntry) -> bool;

    fn focus_lunara(&mut self) -> LunaraResult<()>;

    fn focus_last_app(&mut self) -> LunaraResult<()>;

    fn launch_or_focus(&mut self, app: &AppEntry) -> LunaraResult<()>;
}

pub fn get_wm_from_name(
    // app: AppHandle,
    name: &str,
) -> Result<Box<dyn WindowManagerController + Send>, String> {
    match name {
        // "bspwm" => Ok(Box::new(BspwmWMController::new(app))),
        "bspwm" => Ok(Box::new(BspwmWMController::new())),
        _ => Err(format!("Can't find WM for {name}")),
    }
}
