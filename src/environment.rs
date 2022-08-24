use crate::parsetree::*;
use crate::token::*;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvironmentError {
    #[error("The symbol {id:?} has not been defined in the environment.")]
    UndefinedSymbol { id: Id },

    #[error("Attempted to pop one scope too many")]
    ScopeUnderflow,
}

#[derive(Default, Clone, Debug)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    bindings: HashMap<Id, Expression>,
}

impl Scope {
    pub fn lookup(&self, id: Id) -> Result<Expression, EnvironmentError> {
        if let Some(expr) = self.bindings.get(&id) {
            Ok((*expr).clone())
        } else {
            if let Some(parent) = &self.parent {
                return parent.lookup(id);
            }
            Err(EnvironmentError::UndefinedSymbol { id })
        }
    }
}

pub struct Environment {
    current_scope: Scope,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            current_scope: Scope {
                parent: None,
                bindings: HashMap::default(),
            },
        }
    }

    pub fn push_scope(&mut self) {
        // TODO(@ostera): how can we avoid doing this clone?
        let old_scope = self.current_scope.clone();
        let new_scope = Scope {
            parent: Some(Box::new(old_scope)),
            bindings: HashMap::default(),
        };
        self.current_scope = new_scope;
    }

    pub fn pop_scope(&mut self) -> Result<(), EnvironmentError> {
        if let Some(parent) = self.current_scope.parent.take() {
            self.current_scope = *parent;
            Ok(())
        } else {
            Err(EnvironmentError::ScopeUnderflow)
        }
    }

    pub fn bind(&mut self, id: Id, expr: Expression) {
        self.current_scope.bindings.insert(id, expr);
    }

    pub fn lookup(&self, id: Id) -> Result<Expression, EnvironmentError> {
        self.current_scope.lookup(id)
    }
}

mod tests {
    use super::*;

    #[test]
    fn env_can_lookup_on_parent_scope() {
        let mut env = Environment::new();
        let a = Id("a".to_string());
        let first_str = Expression::LiteralString("hello".to_string());
        env.bind(a.clone(), first_str.clone());
        assert_eq!(env.lookup(a.clone()).unwrap(), first_str);
        env.push_scope();
        assert_eq!(env.lookup(a).unwrap(), first_str);
    }

    #[test]
    fn env_can_not_lookup_on_child_scope() {
        let mut env = Environment::new();
        env.push_scope();
        let a = Id("a".to_string());
        let first_str = Expression::LiteralString("hello".to_string());
        env.bind(a.clone(), first_str.clone());
        assert!(matches!(env.lookup(a.clone()), Ok(expr) if expr == first_str));
        env.pop_scope().unwrap();
        assert!(
            matches!(env.lookup(a.clone()), Err(EnvironmentError::UndefinedSymbol {id}) if id == a)
        );
    }

    #[test]
    fn environment_variable_shadowing() {
        let mut env = Environment::new();
        let a = Id("a".to_string());
        let first_str = Expression::LiteralString("hello".to_string());
        env.bind(a.clone(), first_str.clone());
        assert_eq!(env.lookup(a).unwrap(), first_str);

        env.push_scope();

        let a = Id("a".to_string());
        let second_str = Expression::LiteralString("goodbye".to_string());
        env.bind(a.clone(), second_str.clone());
        assert_eq!(env.lookup(a).unwrap(), second_str);
    }
}
