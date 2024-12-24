use crate::{error, expr::Expr, token::{Literal, Token}, token_type::TokenType};

pub enum Value {
    Nil,
    True,
    False,
    Num(f64),
    Str(String),
}

fn evaluate(expr: Expr) -> Value {
    match expr {
        Expr::Lit(lit) => match lit {
            Literal::None => panic!("unreachable"),
            Literal::Str(str) => Value::Str(str),
            Literal::Num(num) => Value::Num(num),
            Literal::True => Value::True,
            Literal::False => Value::False,
            Literal::Nil => Value::Nil,
        },
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Unary(token, expr) => {
            let right = evaluate(*expr);

            match token.kind {
                TokenType::Minus => -right,
                _ => panic!(""),
            }
        }
        Expr::Unary(_, _) => todo!(),
        Expr::Binary(_, _, _) => todo!(),
    }
}

fn checkNumOperand(token: Token, operand: Value) -> Value {
    match operand {
        Value::Num(num) => Value::Num(-num),
        _ => error("hey"),
    }
}
//pub struct Interpreter {}
//impl Interpreter {
//   fn () {
//
//    }
//}
