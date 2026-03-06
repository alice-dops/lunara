use crate::models::{app_entry::AppEntry, lunara_config::LunaraConfig, snipsets::Snippet};
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use serde_yml::Value;
use std::{env, fs, path::PathBuf};

fn icon_to_data_url(path: &str) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    let encoded = general_purpose::STANDARD.encode(bytes);

    Some(format!("data:image/png;base64,{}", encoded))
}

pub fn config_dir() -> PathBuf {
    if let Ok(p) = env::var("LUNARA_CFG") {
        return PathBuf::from(p);
    }
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("lunara");
    }
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config").join("lunara")
}

fn apps_dir() -> PathBuf {
    config_dir().join("apps")
}

fn snippets_dir() -> PathBuf {
    config_dir().join("snippets")
}

fn config_file() -> PathBuf {
    config_dir().join("config.yml")
}

pub fn load_config() -> LunaraConfig {
    let path = config_file();
    if path.exists() {
        if let Ok(s) = std::fs::read_to_string(&path) {
            match serde_yml::from_str::<LunaraConfig>(&s) {
                Ok(conf) => return conf,
                Err(e) => {
                    eprintln!("Failed to parse config JSON {}: {}", path.display(), e);
                }
            };
        };
    };

    LunaraConfig::default()
}

// pub fn load_file<'a, T>(s: &'a str, yaml: bool) -> Result<T>
// where
//     T: Deserialize<'a>,
// {
//     if yaml {
//         serde_yml::from_str::<T>(&s)
//     } else {
//         serde_json::from_str::<T>(&s)
//     }
// }
//
pub fn load_snippets() -> Vec<Snippet> {
    let dir = snippets_dir();
    let _ = fs::create_dir_all(&dir);

    let mut out = Vec::new();
    let Ok(read_dir) = fs::read_dir(&dir) else {
        return out;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yml") {
            continue;
        }
        if let Ok(s) = fs::read_to_string(&path) {
            // for document in serde_yml::Deserializer::from_str(&s) {
            //     let value = Value::deserialize(document)?;
            // }
            match serde_yml::from_str::<Snippet>(&s) {
                Ok(app) => out.push(app),
                Err(e) => {
                    eprintln!("Failed to parse snippet {}: {}", path.display(), e);
                }
            }
        }
    }

    out.sort_by(|a, b| a.id.to_lowercase().cmp(&b.id.to_lowercase()));
    out
}

pub fn load_apps() -> Vec<AppEntry> {
    let dir = apps_dir();
    let _ = fs::create_dir_all(&dir);

    let mut out = Vec::new();
    let Ok(read_dir) = fs::read_dir(&dir) else {
        return out;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yml") {
            continue;
        }
        if let Ok(s) = fs::read_to_string(&path) {
            if let Ok(mut app) = serde_yml::from_str::<AppEntry>(&s) {
                if let Some(icon_path) = &app.icon {
                    if let Some(data_url) = icon_to_data_url(icon_path) {
                        app.icon = Some(data_url);
                    }
                }
                out.push(app);
            }
        }
    }

    out.sort_by(|a, b| a.id.to_lowercase().cmp(&b.id.to_lowercase()));
    out
}
