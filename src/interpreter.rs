use core::panic;

use crate::{crash, environment::Env, expr::Expr, stmt::Stmt, token_type::TokenType, value::Value};

pub struct Interpreter {
    env: Env,
}
impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for i in 0..statements.len() {
            self.evaluate_stmt(&statements[i]);
        }
    }

    fn evaluate_stmt(&mut self, stmt: &Stmt) {
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
                self.env.insert_value(&token.lexeme, &value);
            }
            Stmt::Block(statements) => {
                self.evaluate_block(statements);
            }
            Stmt::If(first_if, else_ifs, other) => {
                if let Value::True = self.evaluate_expr(&first_if.should_execute) {
                    // execute first if is true
                    self.evaluate_stmt(&first_if.statement);
                } else {
                    // check for other else_ifs
                    let mut else_if_executed = false;
                    for else_if in else_ifs {
                        if let Value::True = self.evaluate_expr(&else_if.should_execute) {
                            self.evaluate_stmt(&else_if.statement);
                            else_if_executed = true;
                            break;
                        }
                    }

                    // execute if there is an else and no else_ifs were executed
                    if !else_if_executed {
                        if let Some(other) = other {
                            self.evaluate_stmt(other);
                        }
                    }
                }
            }
            Stmt::While(expr, statement) => {
                while let Value::True = self.evaluate_expr(expr) {
                   self.evaluate_stmt(statement); 
                }
            }
        }
    }

    fn evaluate_block(&mut self, statements: &Vec<Stmt>) {
        self.env.create_new_child();
        for stmt in statements {
            self.evaluate_stmt(stmt);
        }
        self.env.kill_youngest_child();
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
                    TokenType::Bang => match right.is_true() {
                        Some(bool) => Value::from_bool(!bool),
                        None => crash(
                            token.line,
                            "Uitroepteken kan alleen worden gebruikt op waarheidswaardes, kaaskop",
                        ),
                    },
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
                            crash(op.line, concat!(stringify!($op), " kan alleen worden gebruikt op nummers, kaaskop"))
                        }
                    };
                }

                macro_rules! apply_logic_to_nums {
                    ($type: ident, $op: tt) => {
                        if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                            Value::from_bool(num1 $op num2)
                        } else {
                            crash(op.line, concat!(stringify!($op), " kan alleen worden gebruikt op nummers, kaaskop"));
                        }
                    };
                }

                match op.kind {
                    TokenType::Plus => match (left, right) {
                        (Value::Num(num), Value::Str(str)) => {
                            return Value::Str(format!("{}{}", num, str))
                        }

                        (Value::Str(str), Value::Num(num)) => {
                            return Value::Str(format!("{}{}", str, num))
                        }
                        (Value::Num(num1), Value::Num(num2)) => return Value::Num(num1 + num2),
                        (Value::Str(str1), Value::Str(str2)) => {
                            return Value::Str(format!("{}{}", str1, str2))
                        }
                        _ => crash(
                            op.line,
                            "Plus kan alleen worden gebruikt op nummers en strings, kaaskop.",
                        ),
                    },
                    TokenType::Minus => apply_arith_to_nums!(Minus, -),
                    TokenType::Star => apply_arith_to_nums!(Star, *),
                    TokenType::Slash => apply_arith_to_nums!(Slash, /),

                    TokenType::Caret => match (left, right) {
                        (Value::Num(num1), Value::Num(num2)) => return Value::Num(num1.powf(num2)),
                        _ => crash(
                            op.line,
                            "Caret kan alleen worden gebruikt op nummers, kaaskop.",
                        ),
                    },

                    TokenType::Greater => apply_logic_to_nums!(Greater, >),
                    TokenType::GreaterEqual => apply_logic_to_nums!(GreaterEqual, >=),
                    TokenType::Less => apply_logic_to_nums!(Less, <),
                    TokenType::LessEqual => apply_logic_to_nums!(LessEqaul, <=),
                    TokenType::Equal => apply_logic_to_nums!(Equal, ==),
                    TokenType::EqualEqual => Value::from_bool(Value::is_equal(&left, &right)),
                    TokenType::BangEqual => Value::from_bool(!Value::is_equal(&left, &right)),
                    _ => panic!("Unreachable."),
                }
            }

            // get value from variable name out of hashmap
            Expr::Var(token) => match self.env.get_value(&token) {
                Some(value) => value,
                None => crash(
                    token.line,
                    &format!("{} is een onbekende variabele.", token.lexeme),
                ),
            },

            // overwrite value from variable name out of hashmap
            Expr::Assign(name, expr) => {
                let new_value = self.evaluate_expr(expr);
                if let Err(msg) = self.env.replace_value(name, &new_value) {
                    crash(name.line, &msg)
                }
                new_value
            }
            Expr::Logic(left, op, right) => {
                match op.kind {
                    TokenType::And => {
                        let left = self.evaluate_expr(left).is_true();

                        if let Some(left) = left {
                            let right = self.evaluate_expr(right).is_true();

                            if let Some(right) = right {
                                return Value::from_bool(left && right);
                            } else {
                                crash(op.line, "'en' kan alleen worden gebruikt op waardigheids waarden, kaaskop.");
                            }
                        } else {
                            crash(
                                op.line,
                                "'en' kan alleen worden gebruikt op waardigheids waarden, kaaskop.",
                            );
                        }
                    }

                    TokenType::Or => {
                        match self.evaluate_expr(left).is_true() {
                            Some(left) => {
                                if left {
                                    return Value::True;
                                }
                            }
                            None => crash(
                                op.line,
                                "'of' kan alleen worden gebruikt op waardigheids waarden, kaaskop.",
                            ),
                        }

                        match self.evaluate_expr(right).is_true() {
                            Some(right) => Value::from_bool(right),
                            None => crash(
                                op.line,
                                "'of' kan alleen worden gebruikt op waardigheids waarden, kaaskop.",
                            ),
                        }
                    }
                    _ => panic!("Unreachable."),
                }
            }
        }
    }
}
