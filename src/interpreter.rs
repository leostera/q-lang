use crate::parsetree::*;
use crate::token::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("We were expecting a {expected:?}, but instead found: {found:?}")]
    UnexpectedSymbolFound { expected: Token, found: Token },

    #[error("We were expecting an expression, but instead found: {found:?}")]
    ExpectedExpression { found: Token },

    #[error("We expected {id:?} to be a function but instead found {expr:#?}")]
    CannotCallNonFunctionValue { id: Id, expr: Expression },
}

pub struct Interpreter {}

impl Interpreter {
    /*
    pub fn eval(&mut self, expr: Expression) -> Result<Expression, InterpreterError> {
        match expr {
            Expression::Call { id, args } => match self.env.lookup(id)? {
                Expression::Function(clauses) => {
                    return self.eval_function(clauses, args);
                }
                expr => Err(InterpreterError::CannotCallNonFunctionValue { id, expr }),
            },
            _ => todo!(),
        }
    }

    fn eval_function(
        &self,
        clauses: Vec<FunClause>,
        args: Vec<Expression>,
    ) -> Result<Expression, InterpreterError> {
        let mut args_expr = vec![];
        for arg in args {
            args_exprs.push(self.eval(arg)?);
        }
        let clause = self.find_matching_clause(clauses, args_expr)?;
        self.env.push_frame();
        self.env.pop_frame();
    }
    */
}
