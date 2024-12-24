use std::fs;

use crate::stringify;
use crate::{parser::Parser, scanner::Scanner};

pub struct Lox {
    //had_error: bool,
}
impl Lox {
    pub fn new() -> Self {
        //Self { had_error: false }
        Self {}
    }

    pub fn run_prompt(&mut self, source: String) {
        self.run(source);
    }

    pub fn run_file(&mut self, source: &str) {
        let source = fs::read_to_string(source).unwrap();
        let source = source.to_string();
        self.run(source);
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        println!("{:?}", stringify(&expr));
        //for token in tokens {
        //    print!("{}", token.to_string());
        //}
    }
}
