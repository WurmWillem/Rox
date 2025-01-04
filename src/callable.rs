use std::time::{SystemTime, UNIX_EPOCH};

use crate::value::Value;

pub trait Callable: std::fmt::Debug + CallableClone {
    fn call(&self, arguments: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
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
    fn call(&self, _: Vec<Value>) -> Value {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Value::Num(current_time)
    }

    fn arity(&self) -> usize {
        0
    }
}
