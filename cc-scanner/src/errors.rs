use core::fmt;
use pest::error::Error as PestError;
use thiserror::Error;

use crate::parser::Rule;

#[derive(Debug, PartialEq, Error)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub inner: PestError<Rule>,
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum ParseErrorKind {
    #[error("Err")]
    InvalidCommitType,
    #[error("Err")]
    InvalidScopeDelimiter,
    #[error("Err")]
    InvalidTerminalSeparator,
    #[error("Err")]
    InvalidTokenSeparator,
    #[error("Err")]
    InvalidScopeNoun,
    #[error("Err")]
    InvalidDescription,
    #[error("Err")]
    InvalidBody,
    #[error("Err")]
    InvalidFooter,
    #[error("Err")]
    Other,
}

impl From<PestError<Rule>> for ParseError {
    fn from(pest_error: PestError<Rule>) -> Self {
        let kind = match pest_error.variant {
            pest::error::ErrorVariant::ParsingError { ref positives, .. } => {
                if positives.contains(&Rule::commit_type) {
                    ParseErrorKind::InvalidCommitType
                } else if positives.contains(&Rule::scope_token) {
                    ParseErrorKind::InvalidScopeNoun
                } else if positives.contains(&Rule::colon_separator) {
                    ParseErrorKind::InvalidTokenSeparator
                } else if positives.contains(&Rule::description) {
                    ParseErrorKind::InvalidDescription
                } else if positives.contains(&Rule::body) {
                    ParseErrorKind::InvalidBody
                } else if positives.contains(&Rule::footer_token)
                    || positives.contains(&Rule::footer_token_separator)
                {
                    ParseErrorKind::InvalidFooter
                } else {
                    ParseErrorKind::Other
                }
            }
            pest::error::ErrorVariant::CustomError { .. } => ParseErrorKind::Other,
        };

        ParseError {
            kind,
            inner: pest_error,
        }
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.inner)
    }
}
