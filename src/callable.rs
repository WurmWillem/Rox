use std::time::{SystemTime, UNIX_EPOCH};

use crate::{interpreter::Interpreter, stmt::Stmt, value::Value};

pub trait Callable: std::fmt::Debug + CallableClone {
    fn call(&self, arguments: Vec<Value>, interpreter: &mut Interpreter) -> Value;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}
impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Box<dyn Callable> {
        self.clone_box()
    }
}

// trait is necessary for allowing cloning of Callable
pub trait CallableClone {
    fn clone_box(&self) -> Box<dyn Callable>;
}

impl<T> CallableClone for T
where
    T: 'static + Callable + Clone,
{
    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Clock;
impl Callable for Clock {
    fn call(&self, _: Vec<Value>, _: &mut Interpreter) -> Value {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Value::Num(current_time)
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "clock".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    declaration: Stmt,
}
impl Function {
    pub fn new(declaration: Stmt) -> Self {
        Self { declaration }
    }
}
impl Callable for Function {
    fn call(&self, arguments: Vec<Value>, interpreter: &mut Interpreter) -> Value {
        interpreter.env.create_new_child();

        let Stmt::Function { params, body, .. } = &self.declaration else {
            panic!("Unreachable.");
        };

        for i in 0..params.len() {
            interpreter
                .env
                .insert_value(&params[i].lexeme, arguments[i].clone())
        }

        interpreter.evaluate_stmt(&body);

        interpreter.env.kill_youngest_child();
        Value::Nil
    }

    fn arity(&self) -> usize {
        let Stmt::Function { params, .. } = &self.declaration else {
            panic!("Unreachable.");
        };
        params.len()
    }

    fn to_string(&self) -> String {
        let Stmt::Function { name, .. } = &self.declaration else {
            panic!("Unreachable.");
        };
        name.lexeme.clone()
    }
}
