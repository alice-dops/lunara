use crate::{
    config,
    models::snipsets::Snippet,
    system::command::{run_no_output, LunaraResult},
};

pub struct SnippetsManager {
    snippets: Vec<Snippet>,
}

fn run_snippset_all(snippet: &Snippet, button: String) {
    if let Some(e) = snippet
        .executor
        .iter()
        .find(|exe| exe.action_button == button)
    {
        if let (Some(cmd), Some(args)) = (e.cmd.clone(), e.args.clone()) {
            let _ = run_no_output(cmd.as_str(), &args);
        }
    }
}

impl SnippetsManager {
    pub fn new() -> Self {
        Self {
            snippets: Vec::new(),
        }
    }
    pub fn snippets(&self) -> &[Snippet] {
        &self.snippets
    }

    pub fn reload_config(&mut self) -> bool {
        let new_apps = config::loader::load_snippets();
        let changed = new_apps != self.snippets;
        self.snippets = new_apps;

        changed
    }

    pub fn run_snipsset(&self, id: String, button: String) -> LunaraResult<()> {
        if let Some(s) = self.snippets.iter().find(|snip| snip.id == id) {
            run_snippset_all(s, button);
            Ok(())
        } else {
            Ok(())
        }
    }
}
