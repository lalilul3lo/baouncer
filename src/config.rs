use log::debug;
use miette::miette;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
    sync::Arc,
};
use thiserror::Error;
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct CommitType {
    pub name: String,
    pub description: String,
    pub emoji: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub commit_types: HashMap<String, CommitType>,
    pub scope: bool,
    pub body: bool,
    pub is_breaking: bool,
    pub issues: bool,
    pub footer: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TomlConfig {
    pub commit_types: Vec<CommitType>,
    pub scope: bool,
    pub body: bool,
    pub is_breaking: bool,
    pub issues: bool,
    pub footer: bool,
}
impl Config {
    fn new() -> Self {
        // add default commit types
        let mut commit_types = vec![
            CommitType {
                name: "feat".to_string(),
                description: "A new feature".to_string(),
                emoji: Some("🎁".to_string()),
            },
            CommitType {
                name: "fix".to_string(),
                description: "A bug fix".to_string(),
                emoji: Some("🐛".to_string()),
            },
        ];
        // add conventional commit style types if enabled
        if cfg!(feature = "conventional_types") {
            commit_types.append(&mut vec![
                CommitType {
                    name: "chore".to_string(),
                    description: "Other changes that don't modify src or test files".to_string(),
                    emoji: Some("🧹".to_string()),
                },
                CommitType {
                    name: "docs".to_string(),
                    description: "Documentation only changes".to_string(),
                    emoji: Some("📚".to_string()),
                },
                CommitType {
                    name: "style".to_string(),
                    description: "Changes that do not affect the meaning of the code".to_string(),
                    emoji: Some("💅".to_string()),
                },
                CommitType {
                    name: "perf".to_string(),
                    description: "A code change that improves performance".to_string(),
                    emoji: Some("⚡️".to_string()),
                },
                CommitType {
                    name: "refactor".to_string(),
                    description: "A code change that neither fixes a bug nor adds a feature"
                        .to_string(),
                    emoji: Some("♻️".to_string()),
                },
                CommitType {
                    name: "build".to_string(),
                    description: "Changes that affect the build system or external dependencies."
                        .to_string(),
                    emoji: Some("🛠️".to_string()),
                },
                CommitType {
                    name: "ci".to_string(),
                    description: "Changes to our CI configuration files and scripts.".to_string(),
                    emoji: Some("⚙️".to_string()),
                },
                CommitType {
                    name: "revert".to_string(),
                    description: "Reverts a previous commit.".to_string(),
                    emoji: Some("⏮️".to_string()),
                },
                CommitType {
                    name: "test".to_string(),
                    description: "Adding missing tests or correcting existing tests".to_string(),
                    emoji: Some("✅".to_string()),
                },
            ])
        }

        let mut commit_types_hash = HashMap::new();

        for commit_type in commit_types {
            commit_types_hash.insert(commit_type.name.clone(), commit_type);
        }

        Self {
            commit_types: commit_types_hash,
            scope: true,
            body: true,
            is_breaking: true,
            issues: true,
            footer: true,
        }
    }

    fn merge_commit_types(&mut self, config: TomlConfig) {
        if !config.commit_types.is_empty() {
            for commit_type in config.commit_types {
                if self.commit_types.contains_key(&commit_type.name) {
                    // if given commit type, remove default and insert passed in commit type
                    self.commit_types.remove(&commit_type.name);

                    self.commit_types
                        .insert(commit_type.name.clone(), commit_type);
                } else {
                    // otherwise just insert entry
                    self.commit_types
                        .insert(commit_type.name.clone(), commit_type);
                }
            }
        }

        self.scope = config.scope;

        self.body = config.body;

        self.is_breaking = config.is_breaking;

        self.issues = config.issues;

        self.footer = config.footer;
    }
}

#[derive(Debug, Error, Clone)]
pub enum ConfigError {
    #[error("I/O error reading {path:?}: {source}")]
    IoError {
        path: PathBuf,
        #[source]
        source: Arc<std::io::Error>,
    },

    #[error("TOML parse error in {path:?}: {source}")]
    TomlError {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Unknown error: {0}")]
    Other(String),
}

pub fn init() -> Result<Config, ConfigError> {
    let mut base_config = Config::new();

    let mut config_paths: Vec<PathBuf> = vec![];

    if let Some(home_dir) = dirs::home_dir() {
        let xdg_config_dir = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home_dir.join(".config"));

        let global_config_path = xdg_config_dir.join(".baouncer.toml");

        // global config
        config_paths.push(global_config_path);
    }

    // project specific config
    config_paths.push(PathBuf::from(".baouncer.toml"));

    for path in &config_paths {
        match fs::read_to_string(path) {
            Ok(contents) => match toml::from_str::<TomlConfig>(&contents) {
                Ok(cfg) => {
                    base_config.merge_commit_types(cfg.clone());
                }
                Err(toml_error) => {
                    let err = ConfigError::TomlError {
                        path: path.clone(),
                        source: toml_error,
                    };

                    debug!("{:?}", miette!(err.clone()));

                    return Err(err);
                }
            },
            Err(io_error) => {
                let err = ConfigError::IoError {
                    path: path.clone(),
                    source: Arc::new(io_error),
                };

                debug!("{:?}", miette!(err));
            }
        }
    }

    Ok(base_config)
}
