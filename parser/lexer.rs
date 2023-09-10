use crate::error::ParseError;
use crate::token::Token;
use logos::Logos;
use miette::SourceSpan;

pub struct Lexer<'source> {
    lexer: logos::Lexer<'source, Token>,
    peeked: Option<Token>,
}

impl<'source> Lexer<'source> {
    pub fn from_source(source: &'source str) -> Self {
        let lexer = Token::lexer(source);
        Self {
            lexer,
            peeked: None,
        }
    }

    pub fn span(&self) -> SourceSpan {
        let range = self.lexer.span();
        (range.start, range.end - range.start).into()
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        if let Some(peeked) = self.peeked.take() {
            Ok(peeked)
        } else {
            match self.lexer.next() {
                None => Err(ParseError::EOF),
                Some(token) => Ok(token),
            }
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.peeked.is_none() {
            self.peeked = self.lexer.next();
        }
        self.peeked.clone()
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let found = self.next()?;
        if found == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedSymbolFound { expected, found })
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn peek_shows_the_next_token() {
        let mut lex = Lexer::from_source("1 2");

        assert_eq!(lex.peek(), Some(Token::Number(1)));
        assert_eq!(lex.peek(), Some(Token::Number(1)));
        assert_eq!(lex.next().unwrap(), Token::Number(1));
        assert_eq!(lex.peek(), Some(Token::Number(2)));
        assert_eq!(lex.next().unwrap(), Token::Number(2));
        assert_eq!(lex.peek(), None);
        assert!(lex.next().is_err());
        assert_eq!(lex.peek(), None);
    }
}
