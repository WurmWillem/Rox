use std::env;

use colored::Colorize;
use expr::Expr;
use lox::Lox;

mod expr;
mod lox;
mod parser;
mod scanner;
mod token;
mod token_type;

fn main() {
    let mut lox = Lox::new();

    let args: Vec<String> = env::args().collect();
      //env::set_var("RUST_BACKTRACE", "1");

    if args.len() == 1 {
        // run lox code from a file
        lox.run_file("file.lox");
    } else {
        // run lox code from a prompt
        let mut input = String::new();
        for i in 1..args.len() {
            let arg = format!("{} ", args[i]);
            input.push_str(&arg);
        }

        println!("{}", input);
        lox.run_prompt(input);
    }
}

pub fn stringify(expr: &Expr) -> String {
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
