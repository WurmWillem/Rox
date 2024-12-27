use crate::{crash, token::Token, value::Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    vars: HashMap<String, Value>,
}
impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
            vars: HashMap::new(),
        }
    }

    pub fn insert_value(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn get_value(&self, token: &Token) -> Value {
        if let Some(value) = self.vars.get(&token.lexeme) {
            value.clone()
        } else if let Some(env) = &self.enclosing {
            env.get_value(token)
        } else {
            crash(
                token.line,
                &format!("{} is een onbekende variabele.", token.lexeme),
            );
        }
    }

    pub fn replace_value(&mut self, name: &Token, new_value: Value) {
        if let Some(old_value) = self.vars.get_mut(&name.lexeme) {
            *old_value = new_value;
        } else if let Some(ref mut env) = self.enclosing {
            env.replace_value(name, new_value);
        } else {
            crash(
                name.line,
                &format!("{} is een onbekende variabele.", name.lexeme),
            );
        }
    }
}
