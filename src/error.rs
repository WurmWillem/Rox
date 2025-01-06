use colored::Colorize;

//use crate::value::Value;

pub fn crash(line: usize, message: &str) -> ! {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    panic!("{}{}{}{}", l, line, i, message);
}

//pub enum RuntimeError {
//    Return { value: Value },
//}
