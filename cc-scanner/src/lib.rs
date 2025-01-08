use conventional_commit::{CommitType, ConventionalCommit, Footer, Scope};
use errors::ParseError;
use parser::{CCScanner, Rule};
use pest::Parser;
pub mod conventional_commit;
pub mod errors;
mod parser;

/// Parses the commit type (e.g., "feat", "fix", "docs", etc.) from the provided subject string.
///
/// This is a convenience method that relies on the `CCScanner` rules to extract the commit
/// type from a commit subject line. If parsing fails, it returns a `ParseError`.
///
/// # Arguments
///
/// * `subject` - The subject string from which to parse the commit type.
///
/// # Errors
///
/// Returns a `ParseError` if the commit type cannot be parsed.
///
/// # Examples
///
/// ```rust
/// let commit_type = parse_commit_type("feat: add a new feature")?;
/// assert_eq!(commit_type, CommitType::Feat);
/// ```
pub fn parse_commit_type(subject: &str) -> Result<CommitType, Box<ParseError>> {
    match CCScanner::parse(Rule::commit_type, subject) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            Ok(CommitType::from(pair.as_str()))
        }
        Err(pest_error) => Err(Box::new(ParseError::from(pest_error))),
    }
}

/// Parses the scope of a conventional commit from the provided scope string.
///
/// This is a convenience method that uses the `CCScanner` rules to extract the scope
/// (often the package or module name) from the commit's header section. If parsing fails,
/// it returns a `ParseError`.
///
/// # Arguments
///
/// * `scope` - A string containing the scope substring from a commit header.
///
/// # Errors
///
/// Returns a `ParseError` if the scope fails to parse.
///
/// # Examples
///
/// ```rust
/// let scope = parse_scope("cli")?;
/// assert_eq!(scope.noun, "cli");
/// ```
pub fn parse_scope(scope: &str) -> Result<Scope, ParseError> {
    match CCScanner::parse(Rule::scope_token, scope) {
        Ok(mut rules) => {
            let pairs = rules.next().unwrap();

            Ok(Scope {
                noun: pairs.as_str().to_string(),
            })
        }
        Err(pest_error) => Err(ParseError::from(pest_error)),
    }
}

/// Parses the description portion of the subject line in a conventional commit.
///
/// This convenience method extracts the descriptive text following the commit type (and optional
/// scope). If parsing fails, it returns a `ParseError`.
///
/// # Arguments
///
/// * `subject` - The full subject line, e.g., "feat(scope): add something new".
///
/// # Errors
///
/// Returns a `ParseError` if the description cannot be extracted or is invalid.
///
/// # Examples
///
/// ```rust
/// let description = parse_description("feat(scope): add something new")?;
/// // Inspect `description` as needed
/// ```
pub fn parse_description(subject: &str) -> Result<String, ParseError> {
    match CCScanner::parse(Rule::description, subject) {
        Ok(mut rules) => {
            let pairs = rules.next().unwrap();

            Ok(String::from(pairs.as_str()))
        }
        Err(pest_error) => Err(ParseError::from(pest_error)),
    }
}

/// Parses the body section of a conventional commit message.
///
/// This convenience method extracts the body from a commit message, which typically comes after
/// a blank line following the header. If parsing fails, it returns a `ParseError`.
///
/// # Arguments
///
/// * `subject` - A string slice containing the body section of the commit.
///
/// # Errors
///
/// Returns a `ParseError` if the body fails to parse or is invalid.
///
/// # Examples
///
/// ```rust
/// let body = parse_body("This is the body of the commit")?;
/// // Inspect `body` as needed
/// ```
pub fn parse_body(subject: &str) -> Result<String, ParseError> {
    match CCScanner::parse(Rule::body, subject) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            Ok(String::from(pair.as_str()))
        }
        Err(pest_error) => Err(ParseError::from(pest_error)),
    }
}

/// Parses a single footer from a conventional commit message.
///
/// This convenience method extracts a footer line of the form "key: value" or "BREAKING CHANGE: ...".
/// If parsing fails, it returns a `ParseError`.
///
/// # Arguments
///
/// * `subject` - A string slice that should represent a single footer line.
///
/// # Errors
///
/// Returns a `ParseError` if the footer fails to parse.
///
/// # Examples
///
/// ```rust
/// let footer = parse_footer("Signed-off-by: Some One <some@one.com>")?;
/// // Inspect `footer` as needed
/// ```
pub fn parse_footer(subject: &str) -> Result<Footer, ParseError> {
    match CCScanner::parse(Rule::footer, subject) {
        Ok(mut rules) => {
            let pair = rules.next().unwrap();

            Ok(Footer::from(pair))
        }
        Err(pest_error) => {
            println!("{:#?} HELPME::OOPS", pest_error);
            Err(ParseError::from(pest_error))
        }
    }
}

/// Parses multiple footers from a conventional commit message.
///
/// This convenience method extracts all footer lines (e.g., "Signed-off-by", "BREAKING CHANGE:",
/// "Closes #123") from the commit message. If parsing fails, it returns a `ParseError`.
///
/// # Arguments
///
/// * `footer` - A string containing one or more footer lines.
///
/// # Errors
///
/// Returns a `ParseError` if the footers fail to parse.
///
/// # Examples
///
/// ```rust
/// let footers_str = "Signed-off-by: Some One <some@one.com>\nCo-authored-by: Another <another@some.com>";
/// let footers = parse_footers(footers_str)?;
/// assert_eq!(footers.len(), 2);
/// ```
pub fn parse_footers(footer: &str) -> Result<Vec<Footer>, ParseError> {
    match CCScanner::parse(Rule::footers, footer) {
        Ok(mut rules) => {
            let pairs = rules.next().unwrap();

            let mut footers: Vec<Footer> = vec![];

            for pair in pairs.into_inner() {
                let footer = Footer::from(pair);

                footers.push(footer);
            }

            Ok(footers)
        }
        Err(pest_error) => Err(ParseError::from(pest_error)),
    }
}

/// Parses a full conventional commit message into a `ConventionalCommit` struct.
///
/// This method is the primary entry point for parsing an entire commit message, including
/// the header (commit type, scope, description), body, and one or more footers. If parsing
/// fails, it returns a `ParseError`.
///
/// # Arguments
///
/// * `commit_str` - The complete conventional commit message as a string slice.
///
/// # Returns
///
/// A `ConventionalCommit` struct containing the parsed header, body, and footers.
///
/// # Errors
///
/// Returns a `ParseError` if the commit message fails to parse according to the
/// conventional commit grammar.
///
/// # Examples
///
/// ```rust
/// let commit_message = r#"
/// feat(cli): add a new command
///
/// This introduces a new subcommand called 'serve' that
/// handles local development setup.
///
/// BREAKING CHANGE: The old 'start' command has been removed.
/// "#;
///
/// let conventional_commit = parse_commit(commit_message)?;
/// // Inspect `conventional_commit` as needed
/// ```
pub fn parse_commit(commit_str: &str) -> Result<ConventionalCommit, ParseError> {
    let mut commit = ConventionalCommit::default();

    match CCScanner::parse(Rule::conventional_commit, commit_str) {
        Ok(mut rules) => {
            let pairs = rules.next().unwrap();

            for pair in pairs.into_inner() {
                match pair.as_rule() {
                    Rule::header => commit.set_header(pair),
                    Rule::body => commit.set_body(String::from(pair.as_str())),
                    Rule::footers => {
                        for inner_pair in pair.into_inner() {
                            let footer = Footer::from(inner_pair);

                            commit.set_footer(footer);
                        }
                    }
                    _ => {}
                }
            }

            Ok(commit)
        }
        Err(pest_error) => Err(ParseError::from(pest_error)),
    }
}
