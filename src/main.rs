use baouncer::{
    command_line,
    config::{self, ConfigPrompt},
    git, logger,
    prompt::{
        body, breaking_change, commit_type, confirm_commit, footers, issues, scope, subject,
        Prompts,
    },
};
use cc_scanner::{conventional_commit::ConventionalCommit, parse_commit};
use miette::{miette, Result};

fn main() -> Result<()> {
    let cli = command_line::interface();

    let matches = cli.get_matches();

    logger::init(matches.get_flag("debug"), matches.get_flag("verbose"));

    let config =
        config::init(matches.get_flag("conventional_types")).map_err(|err| miette!("{}", err))?;

    match matches.subcommand() {
        Some(("commit", _)) => {
            let mut commit = ConventionalCommit::default();

            let mut sorted_prompts: Vec<ConfigPrompt> = config
                .prompts
                .values()
                .cloned() // Clone each Prompt so we own it
                .collect();

            sorted_prompts.sort_by_key(|prompt| prompt.order);

            for prompt in sorted_prompts {
                match prompt.kind {
                    Prompts::Type => {
                        commit.set_commit_type(commit_type(config.commit_types.clone())?);
                    }
                    Prompts::Scope => {
                        if let Some(choice) = scope()? {
                            commit.set_scope(choice);
                        }
                    }
                    Prompts::Subject => {
                        commit.set_description(subject()?);
                    }
                    Prompts::Body => {
                        let choice = body()?;

                        if !choice.is_empty() {
                            commit.set_body(choice);
                        }
                    }
                    Prompts::IsBreaking => {
                        commit.set_breaking_change(breaking_change()?);
                    }
                    Prompts::Issues => {
                        commit.set_footers(issues()?);
                    }
                    Prompts::Footers => {
                        if let Some(choice) = footers()? {
                            commit.set_footers(choice);
                        }
                    }
                }
            }

            let parsed_commit = parse_commit(&commit.as_str()).map_err(|err| miette!("{}", err))?;

            match confirm_commit(parsed_commit) {
                Ok(choice) => {
                    if choice {
                        git::commit(commit).map_err(|err| miette!("{}", err))?;
                    }
                }
                Err(error) => {
                    eprintln!("{}", error)
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
