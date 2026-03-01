use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub cmd: Vec<String>,
    pub wm_class: Option<String>,
    pub icon: Option<String>,
    pub bspwm_desktop: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AppView {
    pub app: AppEntry,
}
// impl AppEntry {
//    fn start()
// }
