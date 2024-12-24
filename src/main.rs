use std::{io, fs};

use colored::Colorize;
use expr::Expr;
use parser::Parser;
use scanner::Scanner;

mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

fn main() {
    let mut lox = Lox::new();
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    lox.run("file.lox");
}

struct Lox {
    //had_error: bool,
}

impl Lox {
    fn new() -> Self {
        //Self { had_error: false }
        Self {}
    }

    fn run(&mut self, source: &str) {
        let source = fs::read_to_string(source).unwrap();
        let source = source.to_string();
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

fn stringify(expr: &Expr) -> String {
    match expr {
        Expr::Lit(lit) => lit.to_string(),
        Expr::Grouping(expr) => {
            let expr = *expr.clone();
            parenthesize("group".to_owned(), vec![expr])
        }
        Expr::Unary(token, expr) => {
            let expr = *expr.clone();
            parenthesize(token.lexeme.clone(), vec![expr])
        }
        Expr::Binary(left, token, right) => {
            let left = *left.clone();
            let right = *right.clone();
            parenthesize(token.lexeme.clone(), vec![left, right])
        }
    }
}

fn parenthesize(name: String, exprs: Vec<Expr>) -> String {
    let mut out = format!("({}", name.clone());

    for i in 0..exprs.len() {
        out.push_str(" ");
        out.push_str(&stringify(&exprs[i]));
    }

    out.push_str(")");
    out
}

pub fn error(line: usize, message: &str) {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    panic!("{}{}{}{}", l, line, i, message);
    //panic!("[line {}] Error: {}", line, message);
}
