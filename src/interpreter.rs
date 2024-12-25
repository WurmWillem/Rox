use crate::{crash, expr::Expr, token::Literal, token_type::TokenType};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    True,
    False,
    Num(f64),
    Str(String),
}
impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::True => "true".to_string(),
            Value::False => "false".to_string(),
            Value::Num(num) => num.to_string(),
            Value::Str(str) => str.to_string(),
        }
    }

    fn from_lit(lit: &Literal) -> Self {
        match lit {
            Literal::None => panic!("unreachable"),
            Literal::Str(str) => Value::Str(str.clone()),
            Literal::Num(num) => Value::Num(*num),
            Literal::True => Value::True,
            Literal::False => Value::False,
            Literal::Nil => Value::Nil,
        }
    }

    fn from_bool(is_true: bool) -> Value {
        if is_true {
            return Value::True;
        }
        Value::False
    }

    fn is_true(&self) -> bool {
        match self {
            Value::False | Value::Nil => false,
            _ => true,
        }
    }
}

pub fn interpret(expr: Expr) {
    let value = evaluate(expr);
    println!("{}", value.to_string());
}

fn evaluate(expr: Expr) -> Value {
    match expr {
        Expr::Lit(lit) => Value::from_lit(&lit),
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Unary(token, expr) => {
            let right = evaluate(*expr);
            match token.kind {
                TokenType::Minus => match right {
                    Value::Num(num) => Value::Num(-num),
                    _ => crash(token.line, "Minus can only be applied to numbers."),
                },
                TokenType::Bang => Value::from_bool(!right.is_true()),
                _ => panic!("Unreachable."),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = evaluate(*left);
            let right = evaluate(*right);

            macro_rules! apply_arith_to_nums {
                ($type: ident, $op: tt) => {
                    if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                        Value::Num(num1 $op num2)
                    } else {
                        crash(op.line, concat!(stringify!($op), " can only be applied to numbers."))
                    }
                };
            }

            macro_rules! apply_logic_to_nums {
                ($type: ident, $op: tt) => {
                    if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                        Value::from_bool(num1 $op num2)
                    } else {
                        crash(op.line, concat!(stringify!($op), " can only be applied to numbers."));
                    }
                };
            }

            match op.kind {
                TokenType::Plus => {
                    if let (Value::Num(num1), Value::Num(num2)) = (left.clone(), right.clone()) {
                        return Value::Num(num1 + num2);
                    } else if let (Value::Str(str1), Value::Str(str2)) = (left, right) {
                        return Value::Str(format!("{}{}", str1, str2));
                    }
                    crash(op.line, "Plus can only be applied to numbers.");
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
                _ => panic!("Unreachable"),
            }
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
