use crate::environment::*;
use q_parser::parsetree::*;
use q_parser::token::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("We were expecting a {expected:?}, but instead found: {found:?}")]
    UnexpectedSymbolFound { expected: Token, found: Token },

    #[error("We were expecting an expression, but instead found: {found:?}")]
    ExpectedExpression { found: Token },

    #[error("We expected {id:?} to be a function but instead found {expr:#?}")]
    CannotCallNonFunctionValue { id: Id, expr: Expression },

    #[error("We could not find a clause that matches these arguments")]
    ClauseMatchError,

    #[error(transparent)]
    EnvironmentError(EnvironmentError),
}

pub struct Interpreter {
    program: Module,
    env: Environment,
}

impl Interpreter {
    pub fn new(program: Module) -> Self {
        let mut env = Environment::new();

        for item in &program.items {
            if let ModuleItem::ValueDeclaration(vd) = item {
                env.bind(vd.name.clone(), vd.value.clone())
            }
        }

        Self { program, env }
    }

    pub fn main(mut self) -> Result<(), InterpreterError> {
        self.eval(Expression::Call {
            id: Id("main".to_string()),
            args: vec![Expression::LiteralString("hello world".to_string())],
        })
        .map(|_| ())
    }

    pub fn eval(&mut self, expr: Expression) -> Result<Expression, InterpreterError> {
        match expr {
            Expression::Call { id, args } if id == Id("print".to_string()) => {
                let mut args_exprs = vec![];
                for arg in args {
                    args_exprs.push(self.eval(arg)?);
                }
                print!("{:?}", args_exprs);
                Ok(Expression::LiteralString("ok".to_string()))
            }
            Expression::Call { id, args } => {
                match self
                    .env
                    .lookup(id.clone())
                    .map_err(InterpreterError::EnvironmentError)?
                {
                    Expression::Function(clauses) => self.eval_function(clauses, args),
                    expr => Err(InterpreterError::CannotCallNonFunctionValue { id, expr }),
                }
            }
            Expression::Variable(id) => self
                .env
                .lookup(id)
                .map_err(InterpreterError::EnvironmentError),
            _ => Ok(expr),
        }
    }

    fn eval_function(
        &mut self,
        clauses: Vec<FunClause>,
        args: Vec<Expression>,
    ) -> Result<Expression, InterpreterError> {
        let mut args_exprs = vec![];
        for arg in args {
            args_exprs.push(self.eval(arg)?);
        }
        self.env.push_scope();

        let result = self.bind_matching_clause(clauses, args_exprs)?;

        self.env
            .pop_scope()
            .map_err(InterpreterError::EnvironmentError)?;

        Ok(result)
    }

    fn bind_matching_clause(
        &mut self,
        clauses: Vec<FunClause>,
        args_expr: Vec<Expression>,
    ) -> Result<Expression, InterpreterError> {
        for clause in clauses {
            if clause.args.len() != args_expr.len() {
                continue;
            }

            // TODO(@ostera): make sure the entire pattern matches all the arguments
            // and ONLY THEN bind
            for (pattern, value) in clause.args.iter().zip(args_expr.iter()) {
                if let Pattern::Bind(id) = pattern {
                    self.env.bind(id.clone(), value.clone());
                }
            }

            return self.eval(clause.body);
        }
        Err(InterpreterError::ClauseMatchError)
    }
}

mod tests {
    use q_parser::Parser;

    use super::*;

    #[test]
    fn interpreter_test() {
        let program = r#"
            foo = (A, B) { print(A) }
            main = (Arg) { foo(Arg, Arg) }
        "#;
        let mut parser = Parser::from_string("test_module", program);
        let module = parser.parse().unwrap();

        let mut interpreter = Interpreter::new(module);

        interpreter
            .eval(Expression::Call {
                id: Id("main".to_string()),
                args: vec![Expression::LiteralString("hello world".to_string())],
            })
            .unwrap();

        assert!(false);
    }
}
