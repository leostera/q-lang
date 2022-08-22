use crate::diagnostic::Diagnostic;
use crate::error::ParseError;
use crate::lexer::Lexer;
use crate::parser::Parse;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct Id(pub String);

impl Parse for Id {
    fn from_lexer(lexer: &mut Lexer) -> Result<Self, ParseError> {
        match lexer.next()? {
            Token::Id(id) => Ok(Id(id)),
            token => Err(ParseError::Diagnostic(Diagnostic::UnexpectedSymbolFound {
                expected: Token::Id("some_id".to_string()),
                found: token,
            })),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    LiteralString(String),
}

impl Parse for Expression {
    fn from_lexer(lexer: &mut Lexer) -> Result<Self, ParseError> {
        match lexer.next()? {
            Token::LiteralString(str) => Ok(Expression::LiteralString(str)),
            token => Err(ParseError::Diagnostic(Diagnostic::ExpectedExpression {
                found: token,
            })),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueDeclaration {
    pub name: Id,
    pub value: Expression,
}

impl Parse for ValueDeclaration {
    fn from_lexer(lexer: &mut Lexer) -> Result<Self, ParseError> {
        let name = Id::from_lexer(lexer)?;
        lexer.expect(Token::Equal)?;
        let value = Expression::from_lexer(lexer)
            .map_err(|_| ParseError::Diagnostic(Diagnostic::MissingValueInValueDeclaration))?;

        Ok(Self { name, value })
    }
}

#[derive(Debug, PartialEq)]
pub enum ModuleItem {
    ValueDeclaration(ValueDeclaration),
}

impl Parse for ModuleItem {
    fn from_lexer(lexer: &mut Lexer) -> Result<Self, ParseError> {
        let vd = ValueDeclaration::from_lexer(lexer)?;
        Ok(ModuleItem::ValueDeclaration(vd))
    }
}

#[derive(Debug)]
pub struct Module {
    pub name: Id,
    pub items: Vec<ModuleItem>,
}
