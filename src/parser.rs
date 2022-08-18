use crate::diagnostic::Diagnostic;
use crate::error::*;
use crate::lexer::Lexer;
use crate::parsetree::*;
use std::path::Path;

pub trait Parse {
    fn from_lexer(lexer: &mut Lexer) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub struct Parser {
    source: String,
    module_name: String,
    diagnostics: Vec<Diagnostic>,
}

impl Parser {
    pub fn from_file(filename: &Path) -> Result<Self, ParseError> {
        let source = std::fs::read_to_string(&filename).map_err(ParseError::IOError)?;
        let module_name = filename.file_name().unwrap().to_str().unwrap().to_string();
        let parser = Parser {
            module_name,
            source,
            diagnostics: vec![],
        };
        Ok(parser)
    }

    pub fn from_string(module_name: &str, source: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            source: source.to_string(),
            diagnostics: vec![],
        }
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn parse(&mut self) -> Result<Module, ParseError> {
        let mut lexer = Lexer::from_source(&self.source);

        let mut items = vec![];

        loop {
            match ModuleItem::from_lexer(&mut lexer) {
                Ok(item) => items.push(item),
                Err(ParseError::EOF) => break,
                Err(ParseError::Diagnostic(diag)) => {
                    // TODO(@ostera): skip until the next valid token
                    self.diagnostics.push(diag);
                }
                Err(error) => return Err(error),
            }
        }

        Ok(Module {
            name: Id(self.module_name.clone()),
            items,
        })
    }
}

mod tests {
    use crate::token::Token;

    use super::*;

    #[test]
    fn parse_empty_module() {
        let mut parser = Parser::from_string("test_module", "");
        let module = parser.parse().unwrap();

        assert_eq!(module.name, Id("test_module".to_string()));
        assert_eq!(module.items.len(), 0);
        assert_eq!(module.items, vec![]);
        assert_eq!(parser.diagnostics.len(), 0);
    }

    #[test]
    fn parse_declaration_missing_equals() {
        let mut parser = Parser::from_string(
            "test_module",
            r#"
                Name ? "Q-Lang"
            "#,
        );
        let _ = parser.parse().unwrap();

        assert_eq!(parser.diagnostics.len(), 2);
        assert_eq!(
            parser.diagnostics,
            vec![
                Diagnostic::UnexpectedSymbolFound {
                    expected: Token::Equal,
                    found: Token::Error
                },
                Diagnostic::UnexpectedSymbolFound {
                    expected: Token::Id("some_id".to_string()),
                    found: Token::LiteralString("Q-Lang".to_string())
                }
            ]
        );
    }

    #[test]
    fn parse_module_with_declarations() {
        let mut parser = Parser::from_string(
            "test_module",
            r#"
                Name = "Q-Lang"
            "#,
        );
        let module = parser.parse().unwrap();

        assert_eq!(parser.diagnostics.len(), 0);
        assert_eq!(parser.diagnostics, vec![]);
        assert_eq!(module.name, Id("test_module".to_string()));
        assert_eq!(module.items.len(), 1);
        assert_eq!(
            module.items,
            vec![ModuleItem::ValueDeclaration(ValueDeclaration {
                name: Id("Name".to_string()),
                value: Expression::LiteralString("Q-Lang".to_string())
            })]
        );
    }
}
