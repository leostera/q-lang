use crate::token::*;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, miette::Diagnostic, Debug, PartialEq)]
pub enum ParseError {
    #[error("We were expecting a {expected:?}, but instead found: {found:?}")]
    UnexpectedSymbolFound { expected: Token, found: Token },

    #[error("We were expecting an expression, but instead found: {found:?}")]
    ExpectedExpression { found: Token },

    #[error("When parsing module, we found a declaration without a value.")]
    MissingValueInValueDeclaration,

    #[error("Could not read file {filename:?}")]
    CouldNotReadFile { filename: PathBuf },

    #[error("We reached the end of the file")]
    EOF,
}
