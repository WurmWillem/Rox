use crate::{error::RuntimeErr, token::Token, value::Value};
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

    pub fn print_children(&self, i: usize) {
        for (name, value) in self.vars.clone().into_iter() {
            if name == "n" {
                println!("{}: {:?}", i, value);
            }
        }
        match &self.child {
            Some(child) => child.print_children(i + 1),
            None => return,
        }
    }

    pub fn create_new_child(&mut self) {
        //dbg!("created");
        match &mut self.child {
            Some(child) => child.create_new_child(),
            None => self.child = Some(Box::new(Env::new())),
        }
    }

    pub fn kill_youngest_child(&mut self) -> bool {
        //dbg!("killed");
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
        match self.child {
            Some(ref mut child) => child.insert_value(&name, value),
            None => {
                self.vars.insert(name.clone(), value);
            }
        }
    }

    pub fn get_value(&self, token: &Token) -> Option<Value> {
        if let Some(child) = &self.child {
            if let Some(value) = child.get_value(token) {
                return Some(value);
            }
        }
        self.vars.get(&token.lexeme).cloned()
    }

    pub fn replace_element(
        &mut self,
        name: &Token,
        index: usize,
        new_value: &Value,
    ) -> Result<(), RuntimeErr> {
        if let Some(ref mut child) = self.child {
            if let Ok(()) = child.replace_element(name, index, new_value) {
                return Ok(());
            }
        }

        if let Some(old_value) = self.vars.get_mut(&name.lexeme) {
            if let Value::List(elements) = old_value {
                elements[index] = new_value.clone();
                Ok(())
            } else {
                let msg = format!("'{}' is geen lijst.", name.lexeme);
                Err(RuntimeErr::Err(name.line, msg))
            }
        } else {
            let msg = format!("'{}' is een onbekende variabele.", name.lexeme);
            Err(RuntimeErr::Err(name.line, msg))
        }
    }

    pub fn replace_value(&mut self, name: &Token, new_value: &Value) -> Result<(), RuntimeErr> {
        if let Some(ref mut child) = self.child {
            if let Ok(()) = child.replace_value(name, new_value) {
                return Ok(());
            }
        }
        if let Some(old_value) = self.vars.get_mut(&name.lexeme) {
            *old_value = new_value.clone();
            Ok(())
        } else {
            let msg = format!("'{}' is een onbekende variabele.", name.lexeme);
            Err(RuntimeErr::Err(name.line, msg))
        }
    }
}
