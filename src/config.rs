use crate::prompt::Prompts;
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
pub struct TomlPrompt {
    pub name: String,
    pub order: usize,
}
#[derive(Debug, Deserialize, Clone)]
pub struct TomlConfig {
    pub commit_types: Option<Vec<CommitType>>,
    pub prompts: Vec<TomlPrompt>,
}
#[derive(Debug, Clone)]
pub struct ConfigPrompt {
    pub name: String,
    pub order: usize,
    pub kind: Prompts,
}
#[derive(Debug, Clone)]
pub struct Config {
    pub commit_types: HashMap<String, CommitType>,
    pub prompts: HashMap<String, ConfigPrompt>,
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

        let prompts = vec![
            ConfigPrompt {
                name: "type".to_string(),
                order: 0,
                kind: Prompts::Type,
            },
            ConfigPrompt {
                name: "subject".to_string(),
                order: 1,
                kind: Prompts::Subject,
            },
        ];
        let mut prompts_hash = HashMap::new();
        for prompt in prompts {
            prompts_hash.insert(prompt.name.clone(), prompt);
        }

        Self {
            commit_types: commit_types_hash,
            prompts: prompts_hash,
        }
    }

    fn merge_commit_types(&mut self, config: TomlConfig) {
        if let Some(commit_types) = config.commit_types {
            for commit_type in commit_types {
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
    }

    fn merge_prompts(&mut self, config: TomlConfig) {
        if !config.prompts.is_empty() {
            for prompt in config.prompts {
                if self.prompts.contains_key(&prompt.name) {
                    // if given commit type, remove default
                    self.prompts.remove(&prompt.name);
                }

                self.prompts.insert(
                    prompt.name.clone(),
                    ConfigPrompt {
                        name: prompt.name.clone(),
                        order: prompt.order,
                        kind: Prompts::from(prompt.name.as_str()),
                    },
                );
            }
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum ValidationError {
    #[error("Duplicate prompt of {prompt:?} encountered.")]
    DuplicatePrompts { prompt: String },
    #[error("Prompt {prompt:?} shares the same order of {index:?} held by {existing_prompt:?}")]
    DuplicateOrderIndex {
        prompt: String,
        index: usize,
        existing_prompt: String,
    },
    #[error("Invalid prompt with name {prompt:?} provided.")]
    InvalidPrompt { prompt: String },
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

    #[error("Validation Error: {error:?}")]
    ValidationError { error: ValidationError },

    #[error("Unknown error: {0}")]
    Other(String),
}

pub fn validate_config(cfg: TomlConfig) -> Result<(), ConfigError> {
    let mut name_hash: HashMap<String, bool> = HashMap::new();
    let mut order_hash: HashMap<usize, String> = HashMap::new();

    for prompt in cfg.prompts {
        if name_hash.contains_key(&prompt.name) {
            let err = ConfigError::ValidationError {
                error: ValidationError::DuplicatePrompts {
                    prompt: prompt.name,
                },
            };

            return Err(err);
        } else {
            name_hash.insert(prompt.name.clone(), true);
        }

        if let std::collections::hash_map::Entry::Vacant(e) = order_hash.entry(prompt.order) {
            e.insert(prompt.name.clone());
        } else {
            let existing_prompt = order_hash.get(&prompt.order).unwrap();

            let err = ConfigError::ValidationError {
                error: ValidationError::DuplicateOrderIndex {
                    prompt: prompt.name,
                    index: prompt.order,
                    existing_prompt: existing_prompt.to_string(),
                },
            };

            return Err(err);
        }
    }

    Ok(())
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
                    validate_config(cfg.clone())?;

                    base_config.merge_commit_types(cfg.clone());

                    base_config.merge_prompts(cfg);
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

// TODO: Are unit tests in the file still the thing to do
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that constructing a new `Config` includes default commit types.
    #[test]
    #[cfg(not(feature = "conventional_types"))]
    fn when_creating_a_new_config_it_should_contain_default_commit_types() {
        let config = Config::new();

        assert_eq!(config.commit_types.len(), 2);
        let feat_type = config.commit_types.get("feat").unwrap();
        assert_eq!(feat_type.name, "feat");
        let fix_type = config.commit_types.get("fix").unwrap();
        assert_eq!(fix_type.name, "fix");
    }

    /// Tests that constructing a new `Config` includes default prompts.
    #[test]
    fn when_creating_a_new_config_it_should_contain_default_prompts() {
        let config = Config::new();

        assert_eq!(config.prompts.len(), 2);
        let type_prompt = config.prompts.get("type").unwrap();
        assert_eq!(type_prompt.name, "type");
        let subject_prompt = config.prompts.get("subject").unwrap();
        assert_eq!(subject_prompt.name, "subject");
    }

    #[test]
    #[cfg(feature = "conventional_types")]
    fn when_creating_a_new_config_with_configuration_enabled() {
        let config = Config::new();

        assert_eq!(config.commit_types.len(), 11);
        let feat_type = config.commit_types.get("feat").unwrap();
        assert_eq!(feat_type.name, "feat");
        let fix_type = config.commit_types.get("fix").unwrap();
        assert_eq!(fix_type.name, "fix");
    }

    /// Tests that `merge_commit_types` overrides existing commit types with new definitions.
    #[test]
    fn when_merging_commit_types_it_should_override_existing_definitions() {
        let mut config = Config::new();
        let description = "newly defined description";
        let toml_config = TomlConfig {
            commit_types: Some(vec![CommitType {
                name: "feat".to_string(),
                description: description.to_string(),
                emoji: Some("🌞".to_string()),
            }]),
            prompts: vec![],
        };

        let feat_type = config.commit_types.get("feat").unwrap();
        assert_eq!(feat_type.description, "A new feature");
        assert_eq!(feat_type.emoji, Some("🎁".to_string()));

        config.merge_commit_types(toml_config);

        let feat_type = config.commit_types.get("feat").unwrap();
        assert_eq!(feat_type.description, description);
        assert_eq!(feat_type.emoji, Some("🌞".to_string()))
    }

    /// Tests that `merge_commit_types` appends any commit types not already in the config.
    #[test]
    fn when_merging_commit_types_it_should_append_new_commit_types() {
        let mut config = Config::new();
        let toml_config = TomlConfig {
            commit_types: Some(vec![CommitType {
                name: "docs".to_string(),
                description: "Documentation only changes".to_string(),
                emoji: Some("📚".to_string()),
            }]),
            prompts: vec![],
        };

        config.merge_commit_types(toml_config);

        let doc_type = config.commit_types.get("docs").unwrap();
        assert_eq!(doc_type.description, "Documentation only changes");
        assert_eq!(doc_type.emoji, Some("📚".to_string()))
    }

    /// Tests that `merge_prompts` overrides existing prompts with new definitions.
    #[test]
    fn when_merging_prompts_it_should_override_existing_prompts() {
        let mut config = Config::new();
        let toml_config = TomlConfig {
            commit_types: Some(vec![]),
            prompts: vec![TomlPrompt {
                name: "type".to_string(),
                order: 5,
            }],
        };

        config.merge_prompts(toml_config);

        let type_prompt = config.prompts.get("type").unwrap();
        assert_eq!(type_prompt.order, 5)
    }

    /// Tests that `merge_prompts` adds prompts not previously in the config.
    #[test]
    fn when_merging_prompts_it_should_append_new_prompts() {
        let mut config = Config::new();
        let toml_config = TomlConfig {
            commit_types: Some(vec![]),
            prompts: vec![TomlPrompt {
                name: "footer".to_string(),
                order: 0,
            }],
        };

        assert_eq!(config.prompts.len(), 2);

        config.merge_prompts(toml_config);

        assert_eq!(config.prompts.len(), 3);
        let subject_prompt = config.prompts.get("footer").unwrap();
        assert_eq!(subject_prompt.order, 0)
    }

    /// Tests that `validate_config` succeeds with a properly formed `TomlConfig`.
    #[test]
    fn when_validating_config_with_proper_prompts_it_should_succeed() {
        // let mut config = Config::new();
    }

    /// Tests that `validate_config` rejects a config with duplicate prompt names.
    #[test]
    fn when_duplicate_prompt_names_are_found_it_should_return_a_validation_error() {
        let toml_config = TomlConfig {
            prompts: vec![
                TomlPrompt {
                    name: "scope".to_string(),
                    order: 0,
                },
                TomlPrompt {
                    name: "scope".to_string(),
                    order: 1,
                },
            ],
            commit_types: Some(vec![]),
        };

        let result = validate_config(toml_config);

        assert!(matches!(
            result,
            Err(ConfigError::ValidationError {
                error: ValidationError::DuplicatePrompts { prompt }
            }) if prompt == "scope"
        ));
    }

    /// Tests that `validate_config` rejects a config with duplicate prompt orders.
    #[test]
    fn when_duplicate_prompt_orders_are_found_it_should_return_a_validation_error() {
        let toml_config = TomlConfig {
            prompts: vec![
                TomlPrompt {
                    name: "scope".to_string(),
                    order: 0,
                },
                TomlPrompt {
                    name: "subject".to_string(),
                    order: 0,
                },
            ],
            commit_types: Some(vec![]),
        };

        let result = validate_config(toml_config);

        assert!(matches!(
            result,
            Err(ConfigError::ValidationError {
                error: ValidationError::DuplicateOrderIndex { prompt, index: _, existing_prompt: _ }
            }) if prompt == "subject"
        ));
    }

    /// Tests that `init` uses fallback defaults when no configuration files are found.
    #[test]
    fn when_no_config_files_are_found_init_should_still_return_a_default_config() {
        // TODO: Implement test logic
    }

    /// Tests that `init` fails with a TOML parse error when the config file is invalid.
    #[test]
    fn when_config_file_has_invalid_toml_init_should_return_a_toml_error() {
        // TODO: Implement test logic
    }

    /// Tests that `init` fails with an IO error when the config file is unreadable.
    #[test]
    fn when_config_file_is_unreadable_init_should_return_an_io_error() {
        // TODO: Implement test logic
    }

    /// Tests that `init` properly merges commit types and prompts from the config file.
    #[test]
    fn when_config_files_are_provided_init_should_merge_both_commit_types_and_prompts() {
        // TODO: Implement test logic
    }

    /// Tests that custom `ConfigError` variants can be created or returned for coverage.
    #[test]
    fn when_encountering_various_failures_it_should_produce_the_correct_config_error_variants() {
        // TODO: Implement test logic
    }
}
