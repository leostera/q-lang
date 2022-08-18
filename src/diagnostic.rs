use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Diagnostic {
    UnexpectedSymbolFound { expected: Token, found: Token },

    ExpectedExpression { found: Token },
}
