use core::panic;
use std::collections::HashMap;

use crate::{crash, expr::Expr, stmt::Stmt, token_type::TokenType, value::Value};

pub struct Interpreter {
    vars: HashMap<String, Value>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    pub fn evaluate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.evaluate_expr(expr);
            }
            Stmt::Print(expr) => {
                print!("{}", self.evaluate_expr(expr).to_string());
            }
            Stmt::Println(expr) => {
                println!("{}", self.evaluate_expr(expr).to_string());
            }
            Stmt::Var(token, expr) => {
                let value = self.evaluate_expr(expr);
                self.vars.insert(token.lexeme.clone(), value);
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for i in 0..statements.len() {
            self.evaluate_stmt(&statements[i]);
        }
    }

    pub fn evaluate_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Lit(lit) => Value::from_lit(&lit),
            Expr::Grouping(expr) => self.evaluate_expr(expr),
            Expr::Unary(token, expr) => {
                let right = self.evaluate_expr(expr);

                match token.kind {
                    TokenType::Minus => match right {
                        Value::Num(num) => Value::Num(-num),
                        _ => crash(
                            token.line,
                            "Min kan alleen worden gebruikt voor nummers, kaaskop",
                        ),
                    },
                    TokenType::Bang => Value::from_bool(!right.is_true()),
                    _ => panic!("Unreachable."),
                }
            }
            Expr::Binary(left, op, right) => {
                let left = self.evaluate_expr(left);
                let right = self.evaluate_expr(right);

                macro_rules! apply_arith_to_nums {
                    ($type: ident, $op: tt) => {
                        if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                            Value::Num(num1 $op num2)
                        } else {
                            crash(op.line, concat!(stringify!($op), " kan alleen worden gebruikt voor nummers, kaaskop"))
                        }
                    };
                }

                macro_rules! apply_logic_to_nums {
                    ($type: ident, $op: tt) => {
                        if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                            Value::from_bool(num1 $op num2)
                        } else {
                            crash(op.line, concat!(stringify!($op), " kan alleen worden gebruikt voor nummers, kaaskop"));
                        }
                    };
                }

                match op.kind {
                    TokenType::Plus => {
                        if let (Value::Num(num1), Value::Num(num2)) = (left.clone(), right.clone())
                        {
                            return Value::Num(num1 + num2);
                        } else if let (Value::Str(str1), Value::Str(str2)) = (left, right) {
                            return Value::Str(format!("{}{}", str1, str2));
                        }
                        crash(
                            op.line,
                            "Plus kan alleen worden gebruikt voor nummers en strings, kaaskop.",
                        );
                    }
                    TokenType::Minus => apply_arith_to_nums!(Minus, -),
                    TokenType::Star => apply_arith_to_nums!(Star, *),
                    TokenType::Slash => apply_arith_to_nums!(Slash, /),

                    TokenType::Greater => apply_logic_to_nums!(Greater, >),
                    TokenType::GreaterEqual => apply_logic_to_nums!(GreaterEqual, >=),
                    TokenType::Less => apply_logic_to_nums!(Less, <),
                    TokenType::LessEqual => apply_logic_to_nums!(LessEqaul, <=),
                    TokenType::Equal => apply_logic_to_nums!(Equal, ==),
                    TokenType::EqualEqual => Value::from_bool(is_equal(&left, &right)),
                    TokenType::BangEqual => Value::from_bool(!is_equal(&left, &right)),
                    _ => panic!("Unreachable."),
                }
            }
            Expr::Var(token) => match self.vars.get(&token.lexeme) {
                Some(value) => value.clone(),
                None => crash(
                    token.line,
                    &format!("{} is een onbekende variabele.", token.lexeme),
                ),
            },
            Expr::None => panic!("Unreachable."),
        }
    }
}

fn is_equal(value1: &Value, value2: &Value) -> bool {
    match (value1, value2) {
        (Value::Nil, Value::Nil) => true,
        (Value::True, Value::True) => true,
        (Value::False, Value::False) => true,
        (Value::Num(num1), Value::Num(num2)) => num1 == num2,
        (Value::Str(str1), Value::Str(str2)) => str1 == str2,
        _ => false,
    }
}
