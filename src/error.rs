use crate::value::Value;
use colored::Colorize;

pub fn rox_error(line: usize, message: &str) {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    println!("{}{}{}{}", l, line, i, message);
}

pub fn crash(line: usize, message: &str) -> ! {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    panic!("{}{}{}{}", l, line, i, message);
}

#[derive(Debug, Clone)]
pub enum RoxError {
    Return { value: Value },
    ScanError,
    ParseError { line: usize, msg: String },
}

pub enum RuntimeErr {
    Err(usize, String)
}
