mod commit;
use clap::Command;
use commit::{save_commit, CommitBuilder};
use inquire::{Confirm, Select, Text};

fn cli() -> Command {
    Command::new("cli")
        .about("CLI for creating conventional commits")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("commit").about("Create a conventional commit"))
}

#[derive(Debug)]
struct CommitOption {
    description: String,
    title: String,
    emoji: String,
}

#[derive(Debug)]
struct Question {
    description: String,
    type_: QuestionType,
}

impl Question {
    fn new(description: &str, data: QuestionData) -> Self {
        let question_type = match data {
            QuestionData::Type(options) => QuestionType::Type { options },
            QuestionData::Scope => QuestionType::Scope,
            QuestionData::Subject => QuestionType::Subject,
            QuestionData::Body => QuestionType::Body,
            QuestionData::IsBreaking => QuestionType::IsBreaking,
            QuestionData::Issues => QuestionType::Issues,
        };

        Self {
            description: description.to_string(),
            type_: question_type,
        }
    }
}

#[derive(Debug)]
enum QuestionData {
    Type(Vec<CommitOption>),
    Scope,
    Subject,
    Body,
    IsBreaking,
    Issues,
}

#[derive(Debug)]
enum QuestionType {
    Type { options: Vec<CommitOption> },
    Scope,
    Subject,
    Body,
    IsBreaking,
    Issues,
}

fn main() {
    let questions = vec![
        Question::new(
            "Select the type of change that you're committing",
            QuestionData::Type(vec![
                CommitOption {
                    description: "A new feature".to_string(),
                    title: "feat".to_string(),
                    emoji: "🎉".to_string(),
                },
                CommitOption {
                    description: "A bug fix".to_string(),
                    title: "fix".to_string(),
                    emoji: "🐛".to_string(),
                },
                CommitOption {
                    description: "A documentation change".to_string(),
                    title: "docs".to_string(),
                    emoji: "📚".to_string(),
                },
                CommitOption {
                    description: "Changes that do not affect the meaning of the code (white-space, formatting, etc)".to_string(),
                    title: "style".to_string(),
                    emoji: "📚".to_string(),

                },
                CommitOption {
                    description: "A code change that improves performance".to_string(),
                    title: "perf".to_string(),
                    emoji: "⚡️".to_string(),
                },
                CommitOption {
                    description: "A code refactor".to_string(),
                    title: "refactor".to_string(),
                    emoji: "♻️".to_string(),
                },
                CommitOption {
                    description: "A code change that neither fixes a bug nor adds a feature"
                        .to_string(),
                    title: "style".to_string(),
                    emoji: "💎".to_string(),
                },
                CommitOption {
                    description: "A new test".to_string(),
                    title: "test".to_string(),
                    emoji: "✅".to_string(),
                },
                CommitOption {
                    description: "Changes to our CI configuration files and scripts".to_string(),
                    title: "ci".to_string(),
                    emoji: "⚙️".to_string(),
                },
                CommitOption {
                    description: "Other changes that don't modify src or test files".to_string(),
                    title: "chore".to_string(),
                    emoji: "📦".to_string(),
                },
                CommitOption {
                    description: "Changes to the build process or auxiliary tools and libraries such as documentation generation".to_string(),
                    title: "build".to_string(),
                    emoji: "🛠️".to_string(),
                },
                CommitOption {
                    description: "Reverts a previous commit".to_string(),
                    title: "revert".to_string(),
                    emoji: "🗑️".to_string(),
                }
            ]),
        ),
        Question::new(
            "scope: ", // limit to a single word
            QuestionData::Scope,
        ),
        Question::new(
            "message: ", // is there a character limit?
            QuestionData::Subject,
        ),
        Question::new(
            "body: ", // is there a character limit?
            QuestionData::Body,
        ),
        Question::new(
            "breaking change: ", // is there a character limit?
            QuestionData::IsBreaking,
        ),
        Question::new(
            "issues (comma separated): ", // is there a character limit?
            QuestionData::Issues,
        ),
    ];

    let mut commit_builder = CommitBuilder::new();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("commit", _)) => {
            for question in questions {
                match question.type_ {
                    QuestionType::Type { options } => {
                        let formatted_options: Vec<String> = options
                            .iter()
                            .map(|option| {
                                format!(
                                    "{} - {} ({})",
                                    option.emoji, option.title, option.description
                                )
                            })
                            .collect();

                        let ans = Select::new(&question.description, formatted_options).prompt();
                        match ans {
                            Ok(choice) => {
                                let selected_option = options
                                    .iter()
                                    .find(|&option| {
                                        let formatted_option = format!(
                                            "{} - {} ({})",
                                            option.emoji, option.title, option.description
                                        );
                                        formatted_option == choice
                                    })
                                    .unwrap();

                                println!("You selected the title: {}", selected_option.title);

                                commit_builder.add_type(selected_option.title.clone());
                            }
                            Err(_) => println!("There was an error, please try again"),
                        }
                    }
                    QuestionType::Scope => {
                        let answer = Text::new(&question.description).prompt();

                        match answer {
                            Ok(scope) => {
                                if !scope.is_empty() {
                                    commit_builder.add_scope(scope);
                                }
                            }
                            Err(_) => println!("There was an error, please try again"),
                        }
                    }
                    QuestionType::Subject => {
                        let ans = Text::new(&question.description).prompt();
                        match ans {
                            Ok(subject) => {
                                commit_builder.add_subject(subject);
                            }
                            Err(_) => println!("There was an error, please try again"),
                        }
                    }
                    QuestionType::Body => {
                        let ans = Text::new(&question.description).prompt();
                        match ans {
                            Ok(body) => {
                                if !body.is_empty() {
                                    commit_builder.add_body(body);
                                }
                            }
                            Err(_) => println!("There was an error, please try again"),
                        }
                    }
                    QuestionType::IsBreaking => {
                        let ans = Text::new(&question.description).prompt();
                        match ans {
                            Ok(breaking_change_body) => {
                                if !breaking_change_body.is_empty() {
                                    commit_builder.add_breaking_change(breaking_change_body);
                                }
                            }
                            Err(_) => println!("There was an error, please try again"),
                        }
                    }
                    QuestionType::Issues => {
                        let ans = Text::new(&question.description).prompt();
                        match ans {
                            Ok(issues) => {
                                if !issues.is_empty() {
                                    commit_builder.add_issues(issues);
                                }
                            }
                            Err(_) => println!("There was an error, please try again"),
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    let commit = commit_builder.build();

    match Confirm::new(&format!(
        "Commit message:\n\n👇\n\n{}\n\n👆\n\nDoes this look good? [y/n]",
        &commit
    ))
    .prompt()
    {
        Ok(answer) => {
            if answer {
                match save_commit(commit) {
                    Ok(_) => println!("\n\nCommit saved!"),
                    Err(error) => {
                        println!("\n\nThere was an error saving the commit: {}", error)
                    }
                }
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
