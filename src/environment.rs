use crate::{crash, token::Token, value::Value};
use std::collections::HashMap;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    vars: HashMap<String, Value>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            vars: HashMap::new(),
        }
    }

    pub fn insert_value(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn get_value(&mut self, token: &Token) -> Value {
        match self.vars.get(&token.lexeme) {
            Some(value) => value.clone(),
            None => crash(
                token.line,
                &format!("{} is een onbekende variabele.", token.lexeme),
            ),
        }
    }

    pub fn replace_value(&mut self, name: &Token, new_value: Value) {
        match self.vars.get_mut(&name.lexeme) {
            Some(old_value) => *old_value = new_value,
            None => crash(
                name.line,
                &format!("{} is een onbekende variabele.", name.lexeme),
            ),
        }
    }
}
