use crate::value::Value;
use colored::Colorize;

pub fn error(line: usize, message: &str) {
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

pub enum RoxError {
    Return { value: Value },
    ScanError,
}
