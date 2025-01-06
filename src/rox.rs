use std::fs;

use colored::Colorize;
use crate::{ interpreter::Interpreter, parser::Parser, scanner::Scanner};

const PRINT_SCAN_OUTPUT: bool = false;
const PRINT_PARS_OUTPUT: bool = false;
//const PRINT_INTERPRETER :bool = false;

pub struct Rox {
    //had_error: bool,
}
impl Rox {
    pub fn new() -> Self {
        //Self { had_error: false }
        Self {}
    }

    pub fn run_prompt(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => {
                println!("{}", "Scan error(s) detected, aborting.".purple());
                return;
            },
        };

        let mut parser = Parser::new(tokens);
        let expr = match parser.parse_expr() {
            Ok(expr) => expr,
            Err(_) => {
                println!("{}", "Parsing error(s) detected, aborting.".purple());
                return;
            },
        };
        if PRINT_PARS_OUTPUT {
            println!("{}", expr.to_string());
        }

        let mut interpreter = Interpreter::new();
        let value = interpreter.evaluate_expr(&expr);

        println!("{}", value.to_string());
    }

    pub fn run_file(&mut self, source: &str) {
        let source = fs::read_to_string(source).expect("file.rox is niet gevonden. het moet in dezelfde directory als de binary of Cargo.toml zitten.");
        let source = source.to_string();
        self.run(source);
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);

        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => {
                println!("{}", "Scan error(s) detected, aborting.".purple());
                return;
            },
        };

        if PRINT_SCAN_OUTPUT {
            for token in &tokens {
                print!("{}_", token.to_string());
            }
            println!();
        }

        let mut parser = Parser::new(tokens);

        let statements = match parser.parse_statements() {
            Some(statements) => statements,
            None => {
                println!("{}", "Parsing error(s) detected, aborting.".purple());
                return;
            },
        };

        let mut interpreter = Interpreter::new();
        interpreter.interpret(statements);
    }
}
