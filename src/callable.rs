use std::time::{SystemTime, UNIX_EPOCH};

use crate::{error::{rox_error, RuntimeErr}, interpreter::Interpreter, stmt::Stmt, token::Token, value::Value};

pub trait Callable: std::fmt::Debug + CallableClone {
    fn call(
        &self,
        arguments: Vec<Value>,
        interpreter: &mut Interpreter,
    ) -> Result<Value, RuntimeErr>;
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
    fn call(&self, _: Vec<Value>, _: &mut Interpreter) -> Result<Value, RuntimeErr> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(Value::Num(current_time as i64))
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "clock".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Factorial;
impl Callable for Factorial {
    fn call(&self, arguments: Vec<Value>, _: &mut Interpreter) -> Result<Value, RuntimeErr> {
        let mut result = match arguments[0] {
            Value::Num(num) => num,
            _ => {
                return Err(RuntimeErr::Err(
                    0,
                    "Je kan fact(n) alleen gebruiken op nummers.".to_string(),
                ))
            }
        };
        for i in 2..result {
            result *= i;
        }

        Ok(Value::Num(result))
    }

    fn arity(&self) -> usize {
        1
    }

    fn to_string(&self) -> String {
        "fibonacci".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Fibonacci;
impl Callable for Fibonacci {
    fn call(&self, arguments: Vec<Value>, _: &mut Interpreter) -> Result<Value, RuntimeErr> {
        let n = match arguments[0] {
            Value::Num(num) => num as u128,
            _ => {
                return Err(RuntimeErr::Err(
                    0,
                    "Je kan fact(n) alleen gebruiken op nummers.".to_string(),
                ))
            }
        };

        let mut a: u128 = 0;
        let mut b: u128 = 1;

        for _ in 0..n {
            let temp = a;
            a = b;
            b = temp + b;
        }

        let mut result = a as i64;

        if a > i64::max_value() as u128 {
           result = i64::max_value(); 
           rox_error(0, "overvloei is gebeurt in fib functie.");
        }

        Ok(Value::Num(result))
    }

    fn arity(&self) -> usize {
        1
    }

    fn to_string(&self) -> String {
        "fibonacci".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct FunDeclaration {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
impl Callable for FunDeclaration {
    fn call(
        &self,
        arguments: Vec<Value>,
        interpreter: &mut Interpreter,
    ) -> Result<Value, RuntimeErr> {
        interpreter.env.create_new_child();

        for i in 0..self.params.len() {
            interpreter
                .env
                .insert_value(&self.params[i].lexeme, arguments[i].clone())
        }

        for stmt in &self.body {
            if let Err(e) = interpreter.evaluate_stmt(stmt) {
                interpreter.env.kill_youngest_child();
                match e {
                    RuntimeErr::Return { value } => return Ok(value),
                    RuntimeErr::Err(line, msg) => return Err(RuntimeErr::Err(line, msg)),
                }
            }
        }

        Ok(Value::Nil)
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        self.name.lexeme.clone()
    }
}
