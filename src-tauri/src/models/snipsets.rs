use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snippet {
    pub id: String,
    pub name: String,
    pub confirmation: Option<bool>,
    pub icon: String,
    pub executor: Vec<Executor>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Executor {
    pub action_button: String,
    pub cmd: Option<String>,
    pub args: Option<Vec<String>>,
}
