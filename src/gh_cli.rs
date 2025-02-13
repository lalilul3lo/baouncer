use cc_scanner::conventional_commit::{Footer, Separator};
use inquire::{InquireError, MultiSelect};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Issue {
    pub title: String,
    pub number: u64,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

fn get_issues() -> Vec<Issue> {
    use std::process::Command;

    let output_status = Command::new("gh")
        .args(["issue", "list", "--json", "title,number"])
        .output()
        .expect("Failed to execute gh command");

    let output_str =
        String::from_utf8(output_status.stdout).expect("Failed to convert GH output to string");

    serde_json::from_str(&output_str).expect("Failed to parse JSON from gh CLI")
}

pub fn prompt() -> Result<Vec<Footer>, InquireError> {
    if let Some(choices) = MultiSelect::new("Select issues", get_issues()).prompt_skippable()? {
        if !choices.is_empty() {
            let footers = vec![Footer {
                token: "closes".to_string(),
                separator: Separator::from(": "),
                content: choices
                    .iter()
                    .map(|c| format!("#{}", c.number))
                    .collect::<Vec<_>>()
                    .join(", "),
            }];

            return Ok(footers);
        }
    }

    Ok(vec![])
}
