use crate::diagnostic::*;
use thiserror::Error;

#[derive(Error, miette::Diagnostic, Debug)]
pub enum ParseError {
    #[error("diagnostic: {0:?}")]
    Diagnostic(Diagnostic),

    #[error("diagnostic: {0:?}")]
    Diagnostics(Vec<Diagnostic>),

    #[error(transparent)]
    IOError(std::io::Error),

    #[error("We reached the end of the file")]
    EOF,
}
