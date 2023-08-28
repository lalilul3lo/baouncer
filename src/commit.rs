pub struct CommitBuilder {
    commit_type: Option<String>,
    scope: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    breaking_change: Option<String>,
    issues: Option<String>,
}
impl CommitBuilder {
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

    pub fn add_type(&mut self, commit_type: String) -> &mut Self {
        self.commit_type = Some(commit_type);
        self
    }

    pub fn add_scope(&mut self, scope: String) -> &mut Self {
        self.scope = Some(scope);
        self
    }

    pub fn add_subject(&mut self, subject: String) -> &mut Self {
        self.subject = Some(subject);
        self
    }

    pub fn add_body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn add_breaking_change(&mut self, breaking_change: String) -> &mut Self {
        self.breaking_change = Some(breaking_change);
        self
    }

    pub fn add_issues(&mut self, issues: String) -> &mut Self {
        self.issues = Some(issues);
        self
    }

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
            commit.push_str(&format!("\n\nfixes {}", issues));
        }

        if let Some(breaking_change) = &self.breaking_change {
            commit.push_str(&format!("\n\nBREAKING CHANGE: {}", breaking_change));
        }

        commit
    }
}

#[test]
fn test_commit_builder_with_breaking_change() {
    let commit = CommitBuilder::new()
        .add_type("feat".to_string())
        .add_scope("commit".to_string())
        .add_subject("add commit builder".to_string())
        .add_body("add commit builder".to_string())
        .add_breaking_change("add commit builder".to_string())
        .add_issues("add commit builder".to_string())
        .build();

    assert_eq!(
        commit,
        "feat(commit)!: add commit builder\n\nadd commit builder\n\nfixes add commit builder\n\nBREAKING CHANGE: add commit builder"
    );
}

#[test]
fn test_commit_builder_without_breaking_change() {
    let commit = CommitBuilder::new()
        .add_type("feat".to_string())
        .add_scope("commit".to_string())
        .add_subject("add commit builder".to_string())
        .add_body("add commit builder".to_string())
        .add_issues("add commit builder".to_string())
        .build();

    assert_eq!(
        commit,
        "feat(commit): add commit builder\n\nadd commit builder\n\nfixes add commit builder"
    );
}
