use clap::{
    arg, crate_authors, crate_description, crate_name, crate_version, Arg, ArgAction, Command,
};

/// Returns a clap command-line interface
pub fn interface() -> Command {
    Command::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug mode")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("conventional_types")
                .short('c')
                .long("conventional_types")
                .help("Include Angular style commit types")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .subcommand(Command::new("commit").about("Create a conventional commit"))
        .subcommand(
            Command::new("commit-msg-hook")
                .about("Create a pre-commit hook to check conventional commits"),
        )
        .subcommand(
            Command::new("check-commit")
                .about("Create a pre-commit hook to check conventional commits")
                .args(vec![arg!(-m --message <MESSAGE>)]),
        )
}
