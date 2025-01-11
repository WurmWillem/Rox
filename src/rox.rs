use core::panic;
use std::fs;

use crate::{
    error::{crash, RuntimeErr},
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
};
use colored::Colorize;

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
                println!(
                    "{}",
                    "Scanfout(en) gedetecteerd, programma wordt gestopt.".purple()
                );
                return;
            }
        };

        let mut parser = Parser::new(tokens);
        let expr = match parser.parse_expr() {
            Ok(expr) => expr,
            Err(_) => {
                println!(
                    "{}",
                    "Parsingfout(en) gedetecteerd,  programma wordt gestopt.".purple()
                );
                return;
            }
        };
        if PRINT_PARS_OUTPUT {
            println!("{}", expr.to_string());
        }

        let mut interpreter = Interpreter::new();
        let value = match interpreter.evaluate_expr(&expr) {
            Ok(value) => value,
            Err(err) => {
                let RuntimeErr::Err(line, msg) = err else {
                    panic!("Unreachable.");
                };
                crash(line, &msg);
            }
        };

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
                println!(
                    "{}",
                    "Scan error(s) detected, programma wordt gestopt.".purple()
                );
                return;
            }
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
                println!(
                    "{}",
                    "Parsingfout(en) gedetecteerd, programma wordt gestopt.".purple()
                );
                return;
            }
        };

        let mut interpreter = Interpreter::new();
        if interpreter.interpret(statements) {
            println!("{}", "Rentijd fout(en) gedetecteerd.".purple());
        }
    }
}
