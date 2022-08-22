mod error;
mod lexer;
mod parser;
mod parsetree;
mod token;

fn main() -> miette::Result<()> {
    for file in std::env::args().skip(1) {
        let path = std::path::PathBuf::from(&file);
        let mut parser = parser::Parser::from_file(&path)?;
        let _ = parser.parse()?;

        for diag in parser.diagnostics() {
            dbg!(diag);
        }
    }
    Ok(())
}
