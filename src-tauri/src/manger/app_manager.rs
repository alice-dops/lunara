use crate::{config, models::app_entry::AppEntry};

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

    pub fn find_app(&self, id: &str) -> Option<&AppEntry> {
        self.apps.iter().find(|a| a.id == id)
    }
}
