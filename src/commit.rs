use std::{io, process::Command};

/// Struct representing the components of a Git commit message.
pub struct CommitBuilder {
    commit_type: Option<String>,
    scope: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    breaking_change: Option<String>,
    issues: Option<String>,
}
impl CommitBuilder {
    /// Creates a new `CommitBuilder` instance.
    ///
    /// All fields are initialized to `None`.
    pub fn new() -> Self {
        Self {
            commit_type: None,
            scope: None,
            subject: None,
            body: None,
            breaking_change: None,
            issues: None,
        }
    }

    /// Sets the commit type.
    ///
    /// # Arguments
    ///
    /// * `commit_type` - A string representing the type of commit (e.g., "feat", "fix").
    pub fn add_type(&mut self, commit_type: String) -> &mut Self {
        self.commit_type = Some(commit_type);
        self
    }

    /// Sets the scope of the commit.
    ///
    /// # Arguments
    ///
    /// * `scope` - A string representing the scope of the commit.
    pub fn add_scope(&mut self, scope: String) -> &mut Self {
        self.scope = Some(scope);
        self
    }

    /// Sets the commit subject.
    ///
    /// # Arguments
    ///
    /// * `subject` - A string representing the commit subject.
    pub fn add_subject(&mut self, subject: String) -> &mut Self {
        self.subject = Some(subject);
        self
    }

    /// Sets the commit body.
    ///
    /// # Arguments
    ///
    /// * `body` - A string representing the commit body.
    pub fn add_body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    /// Sets the breaking change note.
    ///
    /// # Arguments
    ///
    /// * `breaking_change` - A string representing the breaking change note.
    pub fn add_breaking_change(&mut self, breaking_change: String) -> &mut Self {
        self.breaking_change = Some(breaking_change);
        self
    }

    /// Sets the issues that are closed by this commit.
    ///
    /// # Arguments
    ///
    /// * `issues` - A string representing the issue numbers, separated by commas.
    pub fn add_issues(&mut self, issues: String) -> &mut Self {
        self.issues = Some(issues);
        self
    }

    /// Builds the commit message string.
    ///
    /// This method constructs the commit message according to the set fields.
    ///
    /// # Returns
    ///
    /// Returns a string representing the complete commit message.
    pub fn build(&self) -> String {
        let mut commit = String::new();

        if let Some(commit_type) = &self.commit_type {
            commit.push_str(commit_type);
        }

        if let Some(scope) = &self.scope {
            commit.push_str(&format!("({})", scope));
        }

        if let Some(subject) = &self.subject {
            if self.breaking_change.is_some() {
                commit.push_str(&format!("!: {}", subject));
            } else {
                commit.push_str(&format!(": {}", subject));
            }
        }

        if let Some(body) = &self.body {
            commit.push_str(&format!("\n\n{}", body));
        }

        if let Some(issues) = &self.issues {
            let numbers: Vec<&str> = issues.split(',').collect();
            let formatted_numbers: Vec<String> =
                numbers.iter().map(|&n| format!("#{}", n)).collect();
            let result = format!("closes {}", formatted_numbers.join(", "));

            commit.push_str(&format!("\n\n{}", result));
        }

        if let Some(breaking_change) = &self.breaking_change {
            commit.push_str(&format!("\n\nBREAKING CHANGE: {}", breaking_change));
        }

        commit
    }
}

/// Saves the commit message using `git commit`.
///
/// # Arguments
///
/// * `commit` - A string representing the complete commit message.
///
/// # Errors
///
/// Returns an error if there are unstaged changes or if the commit message is empty.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure.
pub fn save_commit(commit: String) -> io::Result<()> {
    let output_status = Command::new("git").args(["--no-pager", "diff"]).output()?;

    let status_message = String::from_utf8_lossy(&output_status.stdout)
        .trim()
        .to_string();

    if !status_message.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "You have unstaged changes",
        ));
    }

    if commit.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Commit message is empty",
        ));
    }

    let output = Command::new("git")
        .args(["commit", "-m", &commit])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }
}

#[test]
fn test_add_type() {
    let commit = CommitBuilder::new().add_type("feat".to_string()).build();

    assert_eq!(commit, "feat");
}
#[test]
fn test_add_scope() {
    let commit = CommitBuilder::new()
        .add_type("feat".to_string())
        .add_scope("commit".to_string())
        .build();

    assert_eq!(commit, "feat(commit)");
}
#[test]
fn test_add_subject() {
    let commit = CommitBuilder::new()
        .add_type("feat".to_string())
        .add_scope("commit".to_string())
        .add_subject("add commit builder".to_string())
        .build();

    assert_eq!(commit, "feat(commit): add commit builder");
}
#[test]
fn test_add_body() {
    let commit = CommitBuilder::new()
        .add_type("feat".to_string())
        .add_scope("commit".to_string())
        .add_subject("add commit builder".to_string())
        .add_body("add commit builder".to_string())
        .build();

    assert_eq!(
        commit,
        "feat(commit): add commit builder\n\nadd commit builder"
    );
}
#[test]
fn test_add_breaking_change() {
    let commit = CommitBuilder::new()
        .add_type("feat".to_string())
        .add_scope("commit".to_string())
        .add_subject("add commit builder".to_string())
        .add_body("add commit builder".to_string())
        .add_breaking_change("add commit builder".to_string())
        .build();

    assert_eq!(
        commit,
        "feat(commit)!: add commit builder\n\nadd commit builder\n\nBREAKING CHANGE: add commit builder"
    );
}
#[test]
fn test_add_issues() {
    let commit = CommitBuilder::new()
        .add_type("feat".to_string())
        .add_scope("commit".to_string())
        .add_subject("add commit builder".to_string())
        .add_body("add commit builder".to_string())
        .add_breaking_change("add commit builder".to_string())
        .add_issues("1,2,3".to_string())
        .build();

    assert_eq!(
        commit,
        "feat(commit)!: add commit builder\n\nadd commit builder\n\ncloses #1, #2, #3\n\nBREAKING CHANGE: add commit builder"
    );
}
