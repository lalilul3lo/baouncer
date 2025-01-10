use baouncer::{
    command_line, config, git, logger,
    prompt::{confirm_commit, execute_prompts, PromptSubmissions},
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
                    PromptSubmissions::Type { answer } => {
                        commit.set_commit_type(answer);
                    }
                    PromptSubmissions::Scope { answer } => {
                        commit.set_scope(answer);
                    }
                    PromptSubmissions::Subject { answer } => {
                        commit.set_description(answer);
                    }
                    PromptSubmissions::Body { answer } => {
                        commit.set_body(answer);
                    }
                    PromptSubmissions::IsBreaking { answer } => {
                        commit.set_breaking_change(answer);
                    }
                    PromptSubmissions::Issues { answer } => {
                        commit.set_footers(answer);
                    }
                    PromptSubmissions::Footers { answer } => {
                        commit.set_footers(answer);
                    }
                }
            }

            let parsed_commit = parse_commit(&commit.as_str())?;

            match confirm_commit(parsed_commit) {
                Ok(choice) => {
                    if choice {
                        git::commit(commit)?;
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
