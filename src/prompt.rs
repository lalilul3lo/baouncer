use crate::config::Config;
use cc_scanner::{
    conventional_commit::{CommitType, ConventionalCommit, Footer, Scope},
    parse_footers, parse_scope,
};
use colored::Colorize;
use inquire::{error::InquireResult, required, Confirm, Editor, InquireError, Select, Text};

#[cfg(feature = "gh_cli")]
use crate::gh_cli;

#[derive(Clone)]
pub enum InteractivePrompt {
    Type { answer: CommitType },
    Scope { answer: Scope },
    Subject { answer: String },
    Body { answer: String },
    IsBreaking { answer: bool },
    Issues { answer: Vec<Footer> },
    Footers { answer: Vec<Footer> },
}

pub fn execute_prompts(config: Config) -> Result<Vec<InteractivePrompt>, InquireError> {
    let mut prompts: Vec<InteractivePrompt> = vec![InteractivePrompt::Type {
        answer: Select::new(
            "Select the type of change that you're committing",
            CommitType::variants(),
        )
        .prompt()?,
    }];

    if config.scope {
        let mut ok = false;

        while !ok {
            if let Some(choice) = Text::new("scope:")
                .with_help_message("a noun description")
                .prompt_skippable()?
            {
                if !choice.is_empty() {
                    match parse_scope(&choice) {
                        Ok(scope) => {
                            prompts.push(InteractivePrompt::Scope { answer: scope });

                            ok = true;
                        }
                        Err(error) => {
                            let miette_error = miette::Error::new(error.inner.into_miette());

                            ok = false;

                            eprintln!("{:?}", miette_error);
                        }
                    }
                } else {
                    ok = true
                }
            }
        }
    }

    if config.is_breaking {
        if let Some(choice) = Confirm::new("is breaking change:")
            .with_default(false)
            .prompt_skippable()?
        {
            prompts.push(InteractivePrompt::IsBreaking { answer: choice })
        }
    }

    prompts.push(InteractivePrompt::Subject {
        answer: Text::new("subject:")
            .with_validator(required!("subject is required"))
            .prompt()?,
    });

    if config.body {
        if let Some(choice) = Text::new("body: ")
            .with_help_message("contextual information about the code changes")
            .with_formatter(&|submission| {
                if submission.is_empty() {
                    String::from("<skipped>")
                } else {
                    submission.into()
                }
            })
            .prompt_skippable()?
        {
            if !choice.is_empty() {
                prompts.push(InteractivePrompt::Body { answer: choice })
            }
        }
    }

    #[cfg(feature = "gh_cli")]
    if config.issues && cfg!(feature = "gh_cli") {
        let result = gh_cli::prompt()?;

        prompts.push(result)
    }

    if config.footer {
        let mut ok = false;

        while !ok {
            if let Some(ans) = Editor::new("footer:")
                .with_formatter(&|submission| {
                    if submission.is_empty() {
                        String::from("<skipped>")
                    } else {
                        submission.into()
                    }
                })
                .with_help_message("e.g. breaking change")
                .prompt_skippable()?
            {
                if !ans.is_empty() {
                    match parse_footers(&ans) {
                        Ok(footers) => {
                            prompts.push(InteractivePrompt::Issues { answer: footers });
                            ok = true;
                        }
                        Err(error) => {
                            let miette_error = miette::Error::new(error.inner.into_miette());

                            ok = false;

                            eprintln!("{:?}", miette_error);
                        }
                    }
                } else {
                    ok = true;
                }
            }
        }
    }

    Ok(prompts)
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
