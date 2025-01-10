use baouncer::{
    command_line, config, git, logger,
    prompt::{execute_prompts, InteractivePrompt},
};
use cc_scanner::{conventional_commit::ConventionalCommit, parse_commit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = command_line::interface();

    let matches = cli.get_matches();

    logger::init(matches.get_flag("debug"), matches.get_flag("verbose"));

    let config = config::init()?;

    match matches.subcommand() {
        Some(("commit", _)) => {
            let mut commit = ConventionalCommit::default();

            let submissions = execute_prompts(config)?;

            for submission in submissions {
                match submission {
                    InteractivePrompt::Type { answer } => {
                        commit.set_commit_type(answer);
                    }
                    InteractivePrompt::Scope { answer } => {
                        commit.set_scope(answer);
                    }
                    InteractivePrompt::Subject { answer } => {
                        commit.set_description(answer);
                    }
                    InteractivePrompt::Body { answer } => {
                        commit.set_body(answer);
                    }
                    InteractivePrompt::IsBreaking { answer } => {
                        commit.set_breaking_change(answer);
                    }
                    InteractivePrompt::Issues { answer } => {
                        commit.set_footers(answer);
                    }
                    InteractivePrompt::Footers { answer } => {
                        commit.set_footers(answer);
                    }
                }
            }

            if parse_commit(&commit.as_str()).is_ok() {
                match git::commit(commit) {
                    Ok(oid) => {
                        println!("commit: ({:#?}) created", oid);
                    }
                    Err(error) => {
                        eprintln!("{}", error)
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
