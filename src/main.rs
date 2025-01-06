use std::env;

use rox::Rox;

mod callable;
mod environment;
mod error;
mod expr;
mod interpreter;
mod parser;
mod rox;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod value;

fn main() {
    let mut lox = Rox::new();

    let arguments: Vec<String> = env::args().collect();
    env::set_var("RUST_BACKTRACE", "1");

    if arguments.len() == 1 {
        // run lox code from a file
        lox.run_file("file.rox");
    } else {
        // run lox code from a prompt
        let mut input = String::new();
        for argument in arguments {
            let arg = format!("{} ", argument);
            input.push_str(&arg);
        }

        println!("{}", input);
        lox.run_prompt(input);
    }
}
