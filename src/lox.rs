use std::fs;

use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

const PRINT_SCAN_OUTPUT: bool = false;
const PRINT_PARS_OUTPUT: bool = false;
//const PRINT_INTERPRETER :bool = false;

pub struct Lox {
    //had_error: bool,
}
impl Lox {
    pub fn new() -> Self {
        //Self { had_error: false }
        Self {}
    }

    pub fn run_prompt(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();
        if PRINT_PARS_OUTPUT {
            println!("{}", expr.to_string());
        }

        let mut interpreter = Interpreter::new();
        let value = interpreter.evaluate_expr(&expr);

        println!("{}", value.to_string());
    }

    pub fn run_file(&mut self, source: &str) {
        let source = fs::read_to_string(source).expect("file.lox is niet gevonden. het moet in dezelfde directory als de binary of Cargo.toml zitten.");
        let source = source.to_string();
        self.run(source);
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();
        if PRINT_SCAN_OUTPUT {
            for token in &tokens {
                print!("{}_", token.to_string());
            }
            println!();
        }

        let mut parser = Parser::new(tokens);
        let statements = parser.parse_statements();

        let mut interpreter = Interpreter::new();
        interpreter.interpret(statements);
    }
}
