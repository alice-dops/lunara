use std::{collections::HashMap, time::Duration};

use tauri::{AppHandle, Emitter, Manager};
use tokio::time::interval;

use crate::{
    config,
    models::snipsets::{Snippet, SnippetView},
    system::command::{run_no_output, run_optional, LunaraResult},
};

pub struct SnippetsManager {
    snippets: Vec<Snippet>,
    snippets_status: HashMap<String, Option<String>>,
}

fn run_snippset_all(snippet: &Snippet, button: String) {
    if let Some(e) = snippet
        .executor
        .iter()
        .find(|exe| exe.action_button == button)
    {
        if let Some(cmd) = &e.cmd {
            let args = e.args.as_deref().unwrap_or(&[]);
            let _ = run_no_output(cmd.as_str(), args);
        }
    }
}

fn run_snippset_query(snippet: &Snippet) -> Option<String> {
    if let Some(qv) = &snippet.query_vars {
        if let Some(cmd) = &qv.cmd {
            let args = qv.args.as_deref().unwrap_or(&[]);
            return run_optional(cmd, args);
        }
    }
    None
}

pub fn start_snippets_monitor(app: AppHandle) {
    println!("1");

    let mgr = app.state::<std::sync::Mutex<SnippetsManager>>();
    let mgr = mgr.lock().unwrap();

    let jobs: Vec<(String, u64)> = mgr
        .snippets
        .iter()
        .filter_map(|s| {
            let t = s.query_vars.as_ref()?.time?;
            Some((s.id.clone(), t))
        })
        .collect();
    println!("2");

    for (id, t) in jobs {
        println!("Start thread for {:?} each {:?}s", id, t);

        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let mut tick = interval(Duration::from_secs(t));
            loop {
                tick.tick().await;

                // re-lock manager each tick
                let mgr_state = app.state::<std::sync::Mutex<SnippetsManager>>();
                let mut mgr = mgr_state.lock().unwrap();

                if mgr.run_snippet_query_id(id.clone()) {
                    mgr.send_event_updated(&app);
                }
            }
        });
    }
}

impl SnippetsManager {
    pub fn new() -> Self {
        Self {
            snippets: Vec::new(),
            snippets_status: HashMap::new(),
        }
    }

    // pub fn snippets(&self) -> &[Snippet] {
    //     &self.snippets
    // }

    pub fn reload_config(&mut self) -> bool {
        let new_snippets = config::loader::load_snippets();
        let changed = new_snippets != self.snippets;

        // Remove obsolete keys
        self.snippets_status
            .retain(|id, _| new_snippets.iter().any(|s| &s.id == id));

        // Insert missing ones
        for snippet in &new_snippets {
            self.snippets_status
                .entry(snippet.id.clone())
                .or_insert(None);
        }

        self.snippets = new_snippets;
        self.run_all_query_vars();

        changed
    }

    pub fn run_all_query_vars(&mut self) {
        for snippet in &self.snippets {
            let res = run_snippset_query(snippet);
            self.snippets_status.insert(snippet.id.clone(), res);
        }
    }

    fn run_snippet_query_id(&mut self, id: String) -> bool {
        let Some(snippet) = self.snippets.iter().find(|snip| snip.id == id).cloned() else {
            return false;
        };
        self.run_snippet_query(&snippet)
    }

    fn run_snippet_query(&mut self, snippet: &Snippet) -> bool {
        let res = run_snippset_query(snippet);
        let changed = self
            .snippets_status
            .get(&snippet.id)
            .map_or(true, |old| *old != res);

        self.snippets_status.insert(snippet.id.clone(), res);

        changed
    }

    pub fn get_snippet_views(&self) -> Vec<SnippetView> {
        let l = self
            .snippets
            .iter()
            .map(|snippet| SnippetView {
                snippet: snippet.clone(),
                query: self
                    .snippets_status
                    .get(&snippet.id)
                    .cloned()
                    .unwrap_or(None),
            })
            .collect();
        l
    }

    pub fn run_snipsset(
        &mut self,
        app: &AppHandle,
        id: String,
        button: String,
    ) -> LunaraResult<()> {
        let snippet = self.snippets.iter().find(|snip| snip.id == id).cloned(); // requires Snippet: Clone

        if let Some(s) = snippet {
            run_snippset_all(&s, button);
            if self.run_snippet_query(&s) {
                self.send_event_updated(app);
            }
        }

        Ok(())
    }

    fn send_event_updated(&self, app: &AppHandle) {
        let _ = app.emit("lunara://snippets", self.get_snippet_views());
    }
}
