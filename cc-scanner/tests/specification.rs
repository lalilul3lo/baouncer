use cc_scanner::{
    conventional_commit::{CommitType, ConventionalCommit, Footer, Scope, Separator},
    parse_commit,
};
use indoc::indoc;

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L106
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L100
#[test]
fn commit_type_prefix_with_description() {
    let commit = "test: add integration tests";

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("test"),
            scope: None,
            description: "add integration tests".to_string(),
            body: None,
            footers: vec![],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L124
#[test]
fn other_commit_type_prefix() {
    let commit = "evolution: mewtwo";

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("evolution"),
            scope: None,
            description: "mewtwo".to_string(),
            body: None,
            footers: vec![],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L104-L105
#[test]
fn commit_type_prefix_with_scope() {
    let commit = "test(lib): add integration tests";

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("test"),
            scope: Some(Scope {
                noun: "lib".to_string()
            }),
            description: "add integration tests".to_string(),
            body: None,
            footers: vec![],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L100
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L117-L118
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L121-L122
#[test]
fn commit_type_prefix_with_scope_and_optional_breaking_change_indicator() {
    let commit = "feat(node)!: upgrade @hapijs/hapi to v20";

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: Some(Scope {
                noun: "node".to_string()
            }),
            description: "upgrade @hapijs/hapi to v20".to_string(),
            body: None,
            footers: vec![],
            is_breaking_change: true,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L108
#[test]
fn optional_body() {
    let commit = indoc! {"
        feat: a new feature

        some body"
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "a new feature".to_string(),
            body: Some("some body".to_string()),
            footers: vec![],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L109
#[test]
fn multiple_optional_bodies() {
    let commit = indoc! {"
        feat: a new feature

        some body

        more body"
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "a new feature".to_string(),
            body: Some("some body\n\nmore body".to_string()),
            footers: vec![],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L110-L112
#[test]
fn optional_footer() {
    let commit = indoc! {"
        feat: a new feature

        witness: someone"
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "a new feature".to_string(),
            body: None,
            footers: vec![Footer {
                token: "witness".to_string(),
                separator: Separator::from(": "),
                content: "someone".to_string(),
            }],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L110-L112
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L115-L116
#[test]
fn multiple_optional_footers() {
    let commit = indoc! {"
        feat: a new feature

        witness: Ash

        witness: Brock"
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "a new feature".to_string(),
            body: None,
            footers: vec![
                Footer {
                    token: "witness".to_string(),
                    separator: Separator::from(": "),
                    content: "Ash".to_string(),
                },
                Footer {
                    token: "witness".to_string(),
                    separator: Separator::from(": "),
                    content: "Brock".to_string(),
                }
            ],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L110-L112
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L115-L116
#[test]
fn optional_footers_with_pound_separator() {
    let commit = indoc! {"
        feat: a new feature

        witness # Ash"
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "a new feature".to_string(),
            body: None,
            footers: vec![Footer {
                token: "witness".to_string(),
                separator: Separator::from(" #"),
                content: "Ash".to_string(),
            },],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L113-L114
#[test]
fn footer_token_whitespace_separator() {
    let commit = indoc! {"
        feat: a new feature

        Peer-reviewed-by: someone"
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "a new feature".to_string(),
            body: None,
            footers: vec![Footer {
                token: "Peer-reviewed-by".to_string(),
                separator: Separator::from(": "),
                content: "someone".to_string(),
            }],
            is_breaking_change: false,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L113-L114
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L117-L118
#[test]
fn breaking_change_token_dash_separated_variant() {
    let commit = indoc! {"
        feat: upgrade @hapijs/hapi to v20

        BREAKING-CHANGE: In v20 @hapijs/joi has been moved out as a standalone package."
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "upgrade @hapijs/hapi to v20".to_string(),
            body: None,
            footers: vec![Footer {
                token: "BREAKING-CHANGE".to_string(),
                separator: Separator::from(": "),
                content: "In v20 @hapijs/joi has been moved out as a standalone package."
                    .to_string(),
            }],
            is_breaking_change: true,
        }
    )
}

/// Source(s):
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L113-L114
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L117-L118
/// https://github.com/conventional-commits/conventionalcommits.org/blob/7eee1e0757fd47adf33543c40692138bd6eafc8c/content/v1.0.0/index.md?plain=1#L119-L120
#[test]
fn breaking_change_token_whitespace_separated_variant() {
    let commit = indoc! {"
        feat: upgrade @hapijs/hapi to v20

        BREAKING CHANGE: In v20 @hapijs/joi has been moved out as a standalone package."
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::from("feat"),
            scope: None,
            description: "upgrade @hapijs/hapi to v20".to_string(),
            body: None,
            footers: vec![Footer {
                token: "BREAKING CHANGE".to_string(),
                separator: Separator::from(": "),
                content: "In v20 @hapijs/joi has been moved out as a standalone package."
                    .to_string(),
            }],
            is_breaking_change: true,
        }
    )
}
