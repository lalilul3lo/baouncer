use baouncer::{
    command_line,
    config::{self, ConfigArgs, ConfigPrompt},
    git, logger,
    prompt::{
        body, breaking_change, commit_type, confirm_commit, footers, issues, scope, subject,
        Prompts,
    },
};
use cc_scanner::{conventional_commit::ConventionalCommit, parse_commit};
use miette::{miette, Result};

fn main() -> Result<()> {
    // initialize command line interface
    let cli = command_line::interface();

    // parse arguments
    let matches = cli.get_matches();

    // initialize logger
    logger::init(matches.get_flag("debug"), matches.get_flag("verbose"));

    // initialize cli config
    let cfg = config::init(ConfigArgs {
        conventional_types: matches.get_flag("conventional_types"),
        scope: matches.get_flag("scope"),
        body: matches.get_flag("body"),
        is_breaking: matches.get_flag("is_breaking"),
        footers: matches.get_flag("footers"),
        issues: matches.get_flag("issues"),
    })
    .map_err(|err| miette!("{}", err))?;

    // match on subcommand
    match matches.subcommand() {
        Some(("commit", _)) => {
            let mut sorted_prompts: Vec<ConfigPrompt> = cfg.prompts.values().cloned().collect();

            sorted_prompts.sort_by_key(|prompt| prompt.order);

            let mut commit = ConventionalCommit::default();

            for prompt in sorted_prompts {
                match prompt.kind {
                    Prompts::Type => {
                        commit.set_commit_type(commit_type(cfg.commit_types.clone())?);
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

            // validate commit
            let parsed_commit = parse_commit(&commit.as_str()).map_err(|err| miette!("{}", err))?;

            // display commit message and prompt user to write commit or abort
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
