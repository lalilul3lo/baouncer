use cc_scanner::{
    conventional_commit::{CommitType, ConventionalCommit, Footer, Scope, Separator},
    errors::ParseErrorKind,
    parse_body, parse_commit, parse_commit_type, parse_description, parse_footer, parse_scope,
};
use indoc::indoc;

#[test]
fn test_parse_commit_type() {
    let commit = "feat: add a new feature";

    assert_eq!(parse_commit_type(commit).unwrap(), CommitType::Feature)
}

#[test]
fn test_parse_invalid_commit_type() {
    let commit = "@: add a new feature";

    let result = parse_commit_type(commit);

    assert!(result.is_err());

    assert_eq!(result.unwrap_err().kind, ParseErrorKind::InvalidCommitType)
}

#[test]
fn test_parse_scope() {
    let scope = "neovim";

    assert_eq!(
        parse_scope(scope).unwrap(),
        Scope {
            noun: "neovim".to_string()
        }
    );
}

#[test]
fn test_parse_invalid_scope() {
    let scope = "//";

    let result = parse_scope(scope);

    assert!(result.is_err());

    let err = result.unwrap_err();

    assert_eq!(err.kind, ParseErrorKind::InvalidScopeNoun);
}

#[test]
fn test_parse_description() {
    let desc = "add a new feature";

    assert_eq!(
        parse_description(desc).unwrap(),
        String::from("add a new feature")
    )
}

#[test]
fn test_parse_invalid_description() {
    let desc = indoc! {"

        add a new feature
    "};

    let result = parse_description(desc);

    assert!(result.is_err());

    let err = result.unwrap_err();

    assert_eq!(err.kind, ParseErrorKind::InvalidDescription);
}

#[test]
fn test_parse_body() {
    let body = "some commit body";

    assert_eq!(parse_body(body).unwrap(), "some commit body".to_string());
}

#[test]
fn test_multiple_bodies() {
    let body = indoc! {"
        some commit body

        more commit body"
    };
    assert_eq!(
        parse_body(body).unwrap(),
        "some commit body\n\nmore commit body".to_string()
    );
}

#[test]
fn test_parse_footer() {
    let footer = "approved-by: Iroquois Pliskin";

    assert_eq!(
        parse_footer(footer).unwrap(),
        Footer {
            token: "approved-by".to_string(),
            separator: Separator::from(": "),
            content: "Iroquois Pliskin".to_string(),
        }
    );
}

#[test]
fn test_parse_invalid_footer() {
    let footer = "approved-by:Iroquois Pliskin";

    let result = parse_footer(footer);

    assert!(result.is_err());

    assert_eq!(result.unwrap_err().kind, ParseErrorKind::InvalidFooter);
}

#[test]
fn test_parse_invalid_footer_token() {
    let _footer = "approved by: Iroquois Pliskin";

    todo!("ParseErrorKind::InvalidFooterToken")
}

#[test]
fn test_parse_commit_simple() {
    let commit = "feat: a new feature";

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::Feature,
            scope: None,
            body: None,
            description: "a new feature".to_string(),
            footers: vec![],
            is_breaking_change: false
        },
    )
}

#[test]
fn test_parse_commit_complex() {
    let commit = indoc! {
        "
            feat(node)!: update @hapijs to v20

            in anticipation of upgrade to node v14

            BREAKING-CHANGE: In v20 @hapijs/joi has been moved out as a standalone package.
        "
    };

    assert_eq!(
        parse_commit(commit).unwrap(),
        ConventionalCommit {
            commit_type: CommitType::Feature,
            scope: Some(Scope {
                noun: "node".to_string()
            }),
            description: "update @hapijs to v20".to_string(),
            body: Some("in anticipation of upgrade to node v14".to_string()),
            footers: vec![Footer {
                token: "BREAKING-CHANGE".to_string(),
                separator: Separator::from(": "),
                content: "In v20 @hapijs/joi has been moved out as a standalone package."
                    .to_string(),
            }],
            is_breaking_change: true
        },
    )
}
