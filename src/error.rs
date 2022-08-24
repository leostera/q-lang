use crate::token::*;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("We were expecting a {expected:?}, but instead found: {found:?}")]
    UnexpectedSymbolFound { expected: Token, found: Token },

    #[error("We were expecting an expression, but instead found: {found:?}")]
    ExpectedExpression { found: Token },

    #[error("We were expecting a pattern, but instead found: {found:?}")]
    ExpectedPattern { found: Token },

    #[error("When parsing module, we found a declaration without a value.")]
    #[diagnostic()]
    MissingValueInValueDeclaration {
        #[label("We expected a value right here")]
        span: SourceSpan,

        #[source_code]
        src: NamedSource,
    },

    #[error("We reached the end of the file")]
    EOF,
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::UnexpectedSymbolFound {
                    expected: l_expected,
                    found: l_found,
                },
                Self::UnexpectedSymbolFound {
                    expected: r_expected,
                    found: r_found,
                },
            ) => l_expected == r_expected && l_found == r_found,
            (
                Self::ExpectedPattern { found: l_found },
                Self::ExpectedPattern { found: r_found },
            ) => l_found == r_found,
            (
                Self::ExpectedExpression { found: l_found },
                Self::ExpectedExpression { found: r_found },
            ) => l_found == r_found,
            (
                Self::MissingValueInValueDeclaration { span: l_span, .. },
                Self::MissingValueInValueDeclaration { span: r_span, .. },
            ) => l_span == r_span,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Error, Diagnostic, Debug)]
#[error("Multiple Errors")]
#[diagnostic()]
pub struct Diagnostics {
    #[related]
    pub errors: Vec<ParseError>,
}
