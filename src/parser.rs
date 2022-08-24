use crate::error::*;
use crate::lexer::Lexer;
use crate::parsetree::*;
use crate::token::*;
use miette::NamedSource;
use std::path::{Path, PathBuf};

pub struct Parser {
    filename: PathBuf,
    source: String,
    module_name: String,
    diagnostics: Vec<ParseError>,
}

impl Parser {
    pub fn from_file(filename: &Path) -> Result<Self, ParseError> {
        let source = std::fs::read_to_string(&filename).unwrap();
        let module_name = filename.file_name().unwrap().to_str().unwrap().to_string();
        let parser = Parser {
            filename: filename.to_path_buf(),
            module_name,
            source,
            diagnostics: vec![],
        };
        Ok(parser)
    }

    pub fn from_string(module_name: &str, source: &str) -> Self {
        Self {
            filename: PathBuf::from("<none>"),
            module_name: module_name.to_string(),
            source: source.to_string(),
            diagnostics: vec![],
        }
    }

    pub fn diagnostics(self) -> Result<(), Diagnostics> {
        if self.diagnostics.is_empty() {
            Ok(())
        } else {
            Err(Diagnostics {
                errors: self.diagnostics,
            })
        }
    }

    pub fn parse(&mut self) -> Result<Module, ParseError> {
        let mut lexer = Lexer::from_source(&self.source);

        let mut items = vec![];

        while lexer.peek().is_some() {
            match self.parse_module_item(&mut lexer) {
                Ok(item) => items.push(item),
                Err(error) => {
                    // TODO(@ostera): skip until the next valid token
                    self.diagnostics.push(error)
                }
            }
        }

        Ok(Module {
            name: Id(self.module_name.clone()),
            items,
        })
    }

    fn named_source(&self) -> NamedSource {
        NamedSource::new(self.filename.to_str().unwrap(), self.source.clone())
    }

    fn parse_module_item(&self, lexer: &mut Lexer) -> Result<ModuleItem, ParseError> {
        let vd = self.parse_value_declaration(lexer)?;
        Ok(ModuleItem::ValueDeclaration(vd))
    }

    fn parse_value_declaration(&self, lexer: &mut Lexer) -> Result<ValueDeclaration, ParseError> {
        let name = self.parse_id(lexer)?;
        lexer.expect(Token::Equal)?;
        let value = self.parse_expression(lexer).map_err(|_| {
            let span = lexer.span();
            let src = self.named_source();
            ParseError::MissingValueInValueDeclaration { span, src }
        })?;

        Ok(ValueDeclaration { name, value })
    }

    fn parse_id(&self, lexer: &mut Lexer) -> Result<Id, ParseError> {
        match lexer.next()? {
            Token::Id(id) => Ok(Id(id)),
            token => Err(ParseError::UnexpectedSymbolFound {
                expected: Token::Id("some_id".to_string()),
                found: token,
            }),
        }
    }

    fn parse_expression(&self, lexer: &mut Lexer) -> Result<Expression, ParseError> {
        match lexer.peek() {
            Some(Token::Id(_)) => self.parse_function_call(lexer),
            Some(Token::LiteralString(str)) => {
                lexer.next()?;
                Ok(Expression::LiteralString(str))
            }
            Some(Token::ParensLeft) => self.parse_function(lexer),
            Some(token) => Err(ParseError::ExpectedExpression { found: token }),
            None => Err(ParseError::EOF),
        }
    }

    fn parse_function(&self, lexer: &mut Lexer) -> Result<Expression, ParseError> {
        let mut clauses = vec![];

        loop {
            let clause = self.parse_function_clause(lexer)?;
            clauses.push(clause);
            if let Some(Token::Semicolon) = lexer.peek() {
                lexer.next()?;
                continue;
            }
            break;
        }

        Ok(Expression::Function(clauses))
    }

    fn parse_function_clause(&self, lexer: &mut Lexer) -> Result<FunClause, ParseError> {
        let args = self.parse_function_args(lexer)?;
        lexer.expect(Token::BraceLeft)?;
        let body = self.parse_expression(lexer)?;
        lexer.expect(Token::BraceRight)?;
        Ok(FunClause { args, body })
    }

    fn parse_function_call(&self, lexer: &mut Lexer) -> Result<Expression, ParseError> {
        let id = self.parse_id(lexer)?;
        lexer.expect(Token::ParensLeft)?;
        // let args = self.parse_expression(lexer).map_err(|_| {
        lexer.expect(Token::ParensRight)?;
        Ok(Expression::Call { id, args: vec![] })
    }

    fn parse_function_args(&self, lexer: &mut Lexer) -> Result<Vec<Pattern>, ParseError> {
        lexer.expect(Token::ParensLeft)?;
        let mut patterns = vec![];

        loop {
            if let Some(Token::ParensRight) = lexer.peek() {
                break;
            }

            let pattern = self.parse_pattern(lexer)?;
            patterns.push(pattern);

            if let Some(Token::Comma) = lexer.peek() {
                lexer.next()?;
                continue;
            }

            break;
        }

        lexer.expect(Token::ParensRight)?;
        Ok(patterns)
    }

    fn parse_pattern(&self, lexer: &mut Lexer) -> Result<Pattern, ParseError> {
        match lexer.peek() {
            Some(Token::Id(_)) => {
                let id = self.parse_id(lexer)?;
                Ok(Pattern::Bind(id))
            }
            Some(token) => Err(ParseError::ExpectedPattern { found: token }),
            None => Err(ParseError::EOF),
        }
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
                ParseError::UnexpectedSymbolFound {
                    expected: Token::Equal,
                    found: Token::QuestionMark
                },
                ParseError::UnexpectedSymbolFound {
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

    #[test]
    fn parse_module_with_a_function_definition() {
        let mut parser = Parser::from_string(
            "test_module",
            r#"
                Print = () { "Hello" }
            "#,
        );
        let module = parser.parse().unwrap();

        assert_eq!(parser.diagnostics, vec![]);
        assert_eq!(parser.diagnostics.len(), 0);
        assert_eq!(module.name, Id("test_module".to_string()));
        assert_eq!(module.items.len(), 1);
        assert_eq!(
            module.items,
            vec![ModuleItem::ValueDeclaration(ValueDeclaration {
                name: Id("Print".to_string()),
                value: Expression::Function(vec![FunClause {
                    args: vec![],
                    body: Expression::LiteralString("Hello".to_string())
                }])
            })]
        );
    }

    #[test]
    fn parse_module_with_a_function_definition_with_multiple_clauses() {
        let mut parser = Parser::from_string(
            "test_module",
            r#"
                Print =
                  () { "Joe" };
                  (A) { "Robert" };
                  (A, B) { "Mike" };
                  (A, B, ) { "Bogdan" }
            "#,
        );
        let module = parser.parse().unwrap();

        assert_eq!(parser.diagnostics, vec![]);
        assert_eq!(parser.diagnostics.len(), 0);
        assert_eq!(module.name, Id("test_module".to_string()));
        assert_eq!(module.items.len(), 1);
        assert_eq!(
            module.items,
            vec![ModuleItem::ValueDeclaration(ValueDeclaration {
                name: Id("Print".to_string()),
                value: Expression::Function(vec![
                    FunClause {
                        args: vec![],
                        body: Expression::LiteralString("Joe".to_string())
                    },
                    FunClause {
                        args: vec![Pattern::Bind(Id("A".to_string()))],
                        body: Expression::LiteralString("Robert".to_string())
                    },
                    FunClause {
                        args: vec![
                            Pattern::Bind(Id("A".to_string())),
                            Pattern::Bind(Id("B".to_string())),
                        ],
                        body: Expression::LiteralString("Mike".to_string())
                    },
                    FunClause {
                        args: vec![
                            Pattern::Bind(Id("A".to_string())),
                            Pattern::Bind(Id("B".to_string())),
                        ],
                        body: Expression::LiteralString("Bogdan".to_string())
                    }
                ])
            })]
        );
    }

    #[test]
    fn parse_module_with_a_function_call() {
        let mut parser = Parser::from_string(
            "test_module",
            r#"
                Print = (Arg) { Print() }
            "#,
        );
        let module = parser.parse().unwrap();

        assert_eq!(parser.diagnostics, vec![]);
        assert_eq!(parser.diagnostics.len(), 0);
        assert_eq!(module.name, Id("test_module".to_string()));
        assert_eq!(module.items.len(), 1);
        assert_eq!(
            module.items,
            vec![ModuleItem::ValueDeclaration(ValueDeclaration {
                name: Id("Print".to_string()),
                value: Expression::Function(vec![FunClause {
                    args: vec![Pattern::Bind(Id("Arg".to_string()))],
                    body: Expression::Call {
                        id: Id("Print".to_string()),
                        args: vec![]
                    }
                }])
            })]
        );
    }
}
