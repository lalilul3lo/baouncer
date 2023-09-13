use std::env;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use regex::Regex;

pub fn parse_commit(commit: String) -> Result<(), String> {
    let lines: Vec<&str> = commit.trim().split('\n').collect();

    // Check the header
    let header_pattern = r"^(\w+)(\(([^)]+)\))?!?: (.+)$";
    let header_re = Regex::new(header_pattern).unwrap();

    if !header_re.is_match(lines[0]) {
        return Err("Invalid header".to_string());
    }

    // Check body and footers
    let footer_pattern = r"^(.+?): (.+)$";
    let footer_re = Regex::new(footer_pattern).unwrap();

    let mut found_footer = false;

    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue; // empty lines are valid and typically separate body from footer
        }

        if footer_re.is_match(line) {
            found_footer = true;
        } else if !found_footer {
            // If it's not a footer and we haven't found a footer yet, assume it's part of the body
            continue;
        } else {
            return Err("Invalid footer or misplaced content after footer".to_string());
        }
    }

    Ok(())
}

pub fn init_commit_msg_hook() -> std::io::Result<()> {
    // Path to the pre-commit hook
    let git_hook_path = Path::new(".git/hooks/commit-msg");

    // Check if the `.git/hooks` directory exists
    if !git_hook_path.parent().unwrap().exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not find .git/hooks directory. Make sure you are in the root of a Git repository.",
        ));
    }

    // Generate the script
    let crate_name = env!("CARGO_PKG_NAME");
    let script = format!("#!/usr/bin/env sh\n\ncommit_message=$(grep -v '^#' \"$1\")\n\n{} check-commit -m \"$commit_message\"\n", crate_name);

    // Write the script to the file
    let mut file = fs::File::create(&git_hook_path)?;
    file.write_all(script.as_bytes())?;

    // Make the script executable
    let mut permissions = file.metadata()?.permissions();
    permissions.set_mode(0o755); // -rwxr-xr-x
    fs::set_permissions(&git_hook_path, permissions)?;

    println!(
        "Successfully created commit-msg hook at {:?}",
        git_hook_path
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_prefix_must_exist() {
        // Test that a type prefix like "fix" or "feat" must exist
        let commit = String::from("hi");

        assert!(parse_commit(commit).is_err());
    }

    #[test]
    fn test_optional_scope() {
        // Test that a scope may be optionally provided after a type
        let commit = String::from("feat(scope): hello world");

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_required_colon_and_space() {
        // Test that a terminal colon and space must exist after the type or optional scope
        let commit = String::from("feat:hello world");
        assert!(parse_commit(commit).is_err());
    }

    #[test]
    fn test_description_must_exist() {
        // Test that a description must immediately follow the colon and space
        let commit = String::from("feat: ");
        assert!(parse_commit(commit).is_err());
    }

    #[test]
    fn test_optional_longer_commit_body() {
        // Test that a longer commit body may be provided
        let commit = String::from(
            r"
            feat(intro): add new feature

            add authentication
        ",
        );

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_footer_format() {
        // Test the format of a footer, including the token and separator
        let commit = String::from(
            r"
            feat(intro): add new feature

            CLOSES: #11
        ",
        );

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_breaking_changes_in_footer() {
        // Test that breaking changes can be specified in the footer
        //
        let commit = String::from(
            r"
            feat(intro): add new feature

            BREAKING CHANGE: major
        ",
        );

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_breaking_changes_with_footer() {
        // Test that breaking changes can be specified in the body
        let commit = String::from(
            r"
            feat(intro): add new feature

            This is a breaking change

            BREAKING CHANGE: major
        ",
        );

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_breaking_changes_in_prefix() {
        // Test that breaking changes can be specified in the type/scope prefix
        let commit = String::from("feat!: add new breaking change");

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_other_types() {
        // Test that types other than "feat" and "fix" can be used
        let commit = String::from("chore: add new chore");

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_case_insensitivity() {
        // Test that, except for "BREAKING CHANGE", the commits are case-insensitive

        let commit = String::from("FEAT: add new feature");

        assert!(parse_commit(commit).is_ok());
    }

    #[test]
    fn test_synonymous_breaking_change() {
        // Test that "BREAKING-CHANGE" is synonymous with "BREAKING CHANGE"
        let commit = String::from(
            r"
            feat(intro): add new feature

            BREAKING-CHANGE: Oops
        ",
        );

        assert!(parse_commit(commit).is_ok());
    }
}
