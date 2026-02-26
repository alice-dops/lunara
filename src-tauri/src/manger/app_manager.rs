use crate::{
    config,
    models::app_entry::AppEntry,
    system::{bspwm::launch_or_focus, command::LunaraResult},
};

pub struct AppManager {
    apps: Vec<AppEntry>,
}

impl AppManager {
    pub fn new() -> Self {
        Self { apps: Vec::new() }
    }
    pub fn apps(&self) -> &[AppEntry] {
        &self.apps
    }

    pub fn reload_config(&mut self) -> bool {
        let new_apps = config::loader::load_apps();
        let changed = new_apps != self.apps;
        self.apps = new_apps;

        changed
    }

    pub fn run_app(&self, id: String) -> LunaraResult<()> {
        if let Some(app) = self.apps().iter().find(|app| app.id == id) {
            launch_or_focus(app)
        } else {
            Ok(())
        }
    }
}
