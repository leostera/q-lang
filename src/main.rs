mod environment;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod parsetree;
mod token;

fn main() -> miette::Result<()> {
    for file in std::env::args().skip(1) {
        let path = std::path::PathBuf::from(&file);
        let mut parser = parser::Parser::from_file(&path)?;
        let module = parser.parse()?;
        let () = parser.diagnostics()?;
        let interpreter = interpreter::Interpreter::new(module);

        interpreter.main().unwrap();
    }
    Ok(())
}
