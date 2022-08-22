use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[_a-zA-Z]+", |lex| lex.slice().parse())]
    Id(String),

    /// TODO(@ostera): figure out how to get backticks to work here :)
    #[regex("(\"([^\"\\\\]|\\\\.)*\")", |lex| lex.slice()[1..lex.slice().len() - 1].parse())]
    LiteralString(String),

    #[token("=")]
    Equal,

    #[token("?")]
    QuestionMark,

    #[token(",")]
    Comma,

    #[token("[")]
    BracketLeft,

    #[token("]")]
    BracketRight,

    #[token("(")]
    ParensLeft,

    #[token(")")]
    ParensRight,

    #[token("{")]
    BraceLeft,

    #[token("}")]
    BraceRight,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(u64),

    #[regex("[0-9]*[.][0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

mod tests {
    use super::*;

    #[test]
    fn literal_empty_list() {
        let mut lex = Token::lexer("[]");
        assert_eq!(lex.next(), Some(Token::BracketLeft));
        assert_eq!(lex.next(), Some(Token::BracketRight));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn literal_non_empty_list() {
        let mut lex = Token::lexer("[3.14, true]");
        assert_eq!(lex.next(), Some(Token::BracketLeft));
        assert_eq!(lex.next(), Some(Token::Float(3.14)));
        assert_eq!(lex.next(), Some(Token::Comma));
        assert_eq!(lex.next(), Some(Token::Id("true".to_string())));
        assert_eq!(lex.next(), Some(Token::BracketRight));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn literal_string() {
        let mut lex = Token::lexer("\"3.14\"");
        assert_eq!(lex.next(), Some(Token::LiteralString("3.14".to_string())));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn literal_float() {
        let mut lex = Token::lexer("3.14");
        assert_eq!(lex.next(), Some(Token::Float(3.14)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn constant_definition() {
        let mut lex = Token::lexer(
            r#"
            PI = 3.14
            Name = "Q"
        "#,
        );

        assert_eq!(lex.next(), Some(Token::Id("PI".to_string())));
        assert_eq!(lex.next(), Some(Token::Equal));
        assert_eq!(lex.next(), Some(Token::Float(3.14)));
        assert_eq!(lex.next(), Some(Token::Id("Name".to_string())));
        assert_eq!(lex.next(), Some(Token::Equal));
        assert_eq!(lex.next(), Some(Token::LiteralString("Q".to_string())));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn function_definition() {
        let mut lex = Token::lexer(
            r#"
            hello_world = () {
            }
        "#,
        );

        assert_eq!(lex.next(), Some(Token::Id("hello_world".to_string())));
        assert_eq!(lex.next(), Some(Token::Equal));
        assert_eq!(lex.next(), Some(Token::ParensLeft));
        assert_eq!(lex.next(), Some(Token::ParensRight));
        assert_eq!(lex.next(), Some(Token::BraceLeft));
        assert_eq!(lex.next(), Some(Token::BraceRight));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn function_call() {
        let mut lex = Token::lexer("hello_world()");
        assert_eq!(lex.next(), Some(Token::Id("hello_world".to_string())));
        assert_eq!(lex.next(), Some(Token::ParensLeft));
        assert_eq!(lex.next(), Some(Token::ParensRight));
        assert_eq!(lex.next(), None);
    }
}
