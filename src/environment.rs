use crate::{token::Token, value::Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    child: Option<Box<Env>>,
    vars: HashMap<String, Value>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            child: None,
            vars: HashMap::new(),
        }
    }

    pub fn create_new_child(&mut self) {
        match &mut self.child {
            Some(child) => child.create_new_child(),
            None => self.child = Some(Box::new(Env::new())),
        }
    }

    pub fn kill_youngest_child(&mut self) -> bool {
        match &mut self.child {
            Some(child) => {
                if !child.kill_youngest_child() {
                    self.child = None; 
                }
                true
            }
            None => false,
        }
    }

    pub fn insert_global_value(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn insert_value(&mut self, name: &String, value: Value) {
        if let Some(ref mut child) = self.child {
            child.insert_value(&name, value);
            return;
        }
        self.vars.insert(name.clone(), value);
    }

    pub fn get_value(&self, token: &Token) -> Option<Value> {
        if let Some(child) = &self.child {
            if let Some(value) = child.get_value(token) {
                return Some(value);
            }
        }
        self.vars.get(&token.lexeme).cloned()
    }

    pub fn replace_value(&mut self, name: &Token, new_value: &Value) -> Result<(), String> {
        if let Some(ref mut child) = self.child {
            if let Ok(()) = child.replace_value(name, &new_value) {
                return Ok(());
            }
        }
        if let Some(old_value) = self.vars.get_mut(&name.lexeme) {
            *old_value = new_value.clone();
            Ok(())
        } else {
            Err(format!("'{}' is een onbekende variabele.", name.lexeme))
        }
    }
}
