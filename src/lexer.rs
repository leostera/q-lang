use crate::diagnostic::Diagnostic;
use crate::error::ParseError;
use crate::token::Token;
use logos::Logos;

pub struct Lexer<'source> {
    lexer: logos::Lexer<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn from_source(source: &'source str) -> Self {
        let lexer = Token::lexer(source);
        Self { lexer }
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        match self.lexer.next() {
            None => Err(ParseError::EOF),
            Some(token) => Ok(token),
        }
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let found = self.next()?;
        if found == expected {
            Ok(())
        } else {
            Err(ParseError::Diagnostic(Diagnostic::UnexpectedSymbolFound {
                expected,
                found,
            }))
        }
    }
}
