use cc_scanner::{
    conventional_commit::{CommitType, ConventionalCommit, Footer, Scope},
    parse_footers, parse_scope,
};
use colored::Colorize;
use inquire::{error::InquireResult, required, Confirm, Editor, InquireError, Select, Text};
use miette::{miette, Result};

#[cfg(feature = "gh_cli")]
use crate::gh_cli;

#[derive(Debug, Clone)]
pub enum Prompts {
    Type,
    Scope,
    Subject,
    Body,
    IsBreaking,
    Issues,
    Footers,
}
impl From<&str> for Prompts {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "type" => Self::Type,
            "scope" => Self::Scope,
            "subject" => Self::Subject,
            "body" => Self::Body,
            "is_breaking" => Self::IsBreaking,
            "issues" => Self::Issues,
            "footers" => Self::Footers,
            _ => Self::Type, // FIX
        }
    }
}

fn to_miette(err: InquireError) -> miette::Report {
    miette!("{}", err)
}

pub fn commit_type() -> Result<CommitType, miette::Report> {
    Select::new(
        "Select the type of change that you're committing",
        CommitType::variants(),
    )
    .prompt()
    .map_err(to_miette)
}

pub fn scope() -> Result<Option<Scope>, miette::Report> {
    let mut scope: Option<Scope> = None;

    while let Some(choice) = Text::new("scope:")
        .with_help_message("a noun description")
        .prompt_skippable()
        .map_err(to_miette)?
    {
        // If the user provides an empty input, break out
        if choice.is_empty() {
            break;
        }

        match parse_scope(&choice) {
            Ok(answer) => {
                scope = Some(answer);
                // Break after successfully parsing a scope
                break;
            }
            Err(error) => {
                let miette_error = miette::Error::new(error.inner.into_miette());
                eprintln!("{:?}", miette_error);
                // Loop continues, allowing the user to re-enter a valid scope
            }
        }
    }

    Ok(scope)
}

pub fn subject() -> Result<String, miette::Report> {
    Text::new("subject:")
        .with_validator(required!("subject is required"))
        .prompt()
        .map_err(to_miette)
}

pub fn body() -> Result<String, miette::Report> {
    Text::new("body: ")
        .with_help_message("contextual information about the code changes")
        .with_formatter(&|submission| {
            if submission.is_empty() {
                String::from("<skipped>")
            } else {
                submission.into()
            }
        })
        .prompt()
        .map_err(to_miette)
}

pub fn breaking_change() -> Result<bool, miette::Report> {
    Confirm::new("is breaking change:")
        .with_default(false)
        .prompt()
        .map_err(to_miette)
}

pub fn issues() -> Result<Vec<Footer>, miette::Report> {
    if cfg!(feature = "gh_cli") {
        let result = gh_cli::prompt().map_err(to_miette)?;

        Ok(result)
    } else {
        Ok(vec![])
    }
}

pub fn footers() -> Result<Option<Vec<Footer>>, miette::Report> {
    let mut footers: Option<Vec<Footer>> = None;

    while let Some(ans) = Editor::new("footer:")
        .with_formatter(&|submission| {
            if submission.is_empty() {
                String::from("<skipped>")
            } else {
                submission.into()
            }
        })
        .with_help_message("e.g. breaking change")
        .prompt_skippable()
        .map_err(to_miette)?
    {
        if ans.is_empty() {
            break;
        }

        match parse_footers(&ans) {
            Ok(answer) => {
                footers = Some(answer);
                break;
            }
            Err(error) => {
                let miette_error = miette::Error::new(error.inner.into_miette());
                eprintln!("{:?}", miette_error);
            }
        }
    }

    Ok(footers)
}

pub fn confirm_commit(mut commit: ConventionalCommit) -> InquireResult<bool> {
    let fancy_prompt = format!(
        "{} {}\n \n{}\n\n{} {}\n",
        "┌─".bold().blue(),
        "Ready to commit?".bold().blue(),
        commit.as_str(),
        "└─".bold().blue(),
        "Press [Enter] to confirm or [Ctrl + C] to cancel".green()
    );

    Confirm::new(&format!("\n\n{}\n\n", fancy_prompt))
        .with_default(true)
        .prompt()
}
