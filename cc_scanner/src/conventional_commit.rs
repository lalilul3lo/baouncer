use crate::parser::Rule;
use core::fmt;
use pest::iterators::Pair;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, EnumIter)]
pub enum CommitType {
    Feature,
    Bug,
    Chore,
    Revert,
    Perf,
    Doc,
    Style,
    Refactor,
    Test,
    Build,
    Ci,
    Custom(String),
}
// Can you define string enums. From and Display trait implementation might be able to be dried
impl From<&str> for CommitType {
    fn from(commit_type: &str) -> Self {
        match commit_type.to_ascii_lowercase().as_str() {
            "feat" => CommitType::Feature,
            "bug" => CommitType::Bug,
            "chore" => CommitType::Chore,
            "revert" => CommitType::Revert,
            "per" => CommitType::Perf,
            "doc" => CommitType::Doc,
            "style" => CommitType::Style,
            "refactor" => CommitType::Refactor,
            "test" => CommitType::Test,
            "build" => CommitType::Build,
            "ci" => CommitType::Ci,
            other => CommitType::Custom(other.to_string()),
        }
    }
}
impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let commit_type_str = match self {
            CommitType::Feature => "feat",
            CommitType::Bug => "bug",
            CommitType::Chore => "chore",
            CommitType::Revert => "revert",
            CommitType::Perf => "perf",
            CommitType::Doc => "doc",
            CommitType::Style => "style",
            CommitType::Refactor => "refactor",
            CommitType::Test => "test",
            CommitType::Build => "build",
            CommitType::Ci => "ci",
            CommitType::Custom(custom) => custom, // Broken prompts. How does this play with configs
        };
        write!(f, "{}", commit_type_str)
    }
}
impl CommitType {
    pub fn variants() -> Vec<CommitType> {
        CommitType::iter().collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum Separator {
    #[default]
    Colon,
    Pound,
    ColonWithNewline,
}
impl From<&str> for Separator {
    fn from(value: &str) -> Self {
        match value {
            ": " => Self::Colon,
            "# " => Self::Pound,
            ":\n" | ":\r" | ":\r\n" => Self::ColonWithNewline,
            other => unreachable!("Unrecognized footer token separator: `{}`", other),
        }
    }
}
impl fmt::Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = match self {
            Self::Colon => ": ",
            Self::Pound => "# ",
            Self::ColonWithNewline => "\n",
        };
        write!(f, "{}", separator)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Footer {
    pub token: String,
    pub separator: Separator,
    pub content: String,
}
impl Footer {
    pub fn is_breaking_change(&self) -> bool {
        self.token == "BREAKING CHANGE" || self.token == "BREAKING-CHANGE"
    }
}
impl From<Pair<'_, Rule>> for Footer {
    fn from(pairs: Pair<'_, Rule>) -> Self {
        let mut pair = pairs.into_inner();
        println!("{:?}", pair);
        // TODO: Error-handling
        let token = pair.next().unwrap().as_str().to_string();
        let separator = pair.next().unwrap().as_str();
        let content = pair.next().unwrap().as_str().to_string().trim().to_string();

        Footer {
            token,
            content,
            separator: Separator::from(separator),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scope {
    pub noun: String,
}

pub struct Description {}

#[derive(Debug, Eq, PartialEq)]
pub struct ConventionalCommit {
    pub commit_type: CommitType,
    pub scope: Option<Scope>,
    pub description: String,
    pub body: Option<String>,
    pub footers: Vec<Footer>,
    pub is_breaking_change: bool,
}
impl Default for ConventionalCommit {
    fn default() -> Self {
        ConventionalCommit {
            commit_type: CommitType::Feature,
            scope: None,
            description: "".to_string(),
            body: None,
            footers: vec![],
            is_breaking_change: false,
        }
    }
}
impl fmt::Display for ConventionalCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut header = self.commit_type.to_string();

        if let Some(scope) = &self.scope {
            header.push_str(&format!("({})", scope.noun));
        }

        if self.is_breaking_change {
            header.push('!');
        }

        header.push_str(": ");

        if !self.description.is_empty() {
            header.push_str(&self.description);
        }

        let mut parts = vec![header];

        if let Some(body) = &self.body {
            parts.push(body.clone());
        }
        if !self.footers.is_empty() {
            for footer in &self.footers {
                parts.push(format!(
                    "{}{}{}",
                    footer.token, footer.separator, footer.content
                ));
            }
        }

        write!(f, "{}", parts.join("\n\n"))
    }
}
impl ConventionalCommit {
    pub fn as_str(&mut self) -> String {
        self.to_string()
    }

    pub fn set_header(&mut self, pair: Pair<Rule>) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::commit_type => self.set_commit_type(CommitType::from(inner_pair.as_str())),
                Rule::description => self.set_description(String::from(inner_pair.as_str())),
                Rule::scope => {
                    for scope_rule_pair in inner_pair.into_inner() {
                        if scope_rule_pair.as_rule() == Rule::scope_token {
                            self.set_scope(Scope {
                                noun: String::from(scope_rule_pair.as_str()),
                            });
                        }
                    }
                }
                Rule::breaking_change_indicator => {
                    self.set_breaking_change(true);
                }
                _ => {}
            }
        }
    }

    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
    }

    pub fn set_footers(&mut self, footers: Vec<Footer>) {
        for footer in footers {
            self.set_footer(footer);
        }
    }

    pub fn set_commit_type(&mut self, commit_type: CommitType) {
        self.commit_type = commit_type;
    }

    pub fn set_scope(&mut self, scope: Scope) {
        self.scope = Some(scope);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_breaking_change(&mut self, flag: bool) {
        self.is_breaking_change = flag;
    }

    pub fn set_footer(&mut self, footer: Footer) {
        if footer.is_breaking_change() {
            self.is_breaking_change = true
        }

        self.footers.push(footer)
    }
}
