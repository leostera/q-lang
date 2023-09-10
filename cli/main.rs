mod environment;
mod interpreter;

fn main() -> miette::Result<()> {
    for file in std::env::args().skip(1) {
        let path = std::path::PathBuf::from(&file);
        let mut parser = q_parser::Parser::from_file(&path)?;
        let module = parser.parse()?;
        let () = parser.diagnostics().unwrap();
        let interpreter = interpreter::Interpreter::new(module);

        interpreter.main().unwrap();
    }
    Ok(())
}
