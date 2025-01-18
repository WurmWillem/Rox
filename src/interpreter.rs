use crate::{
    callable::{Clock, Factorial, Fibonacci},
    environment::Env,
    error::{rox_error, RuntimeErr},
    expr::Expr,
    stmt::{If, Stmt},
    token::Token,
    token_type::TokenType,
    value::Value,
};

pub struct Interpreter {
    pub env: Env,
}
impl Interpreter {
    pub fn new() -> Self {
        let mut env = Env::new();

        let fact = Value::Callable(Box::new(Factorial {}));
        env.insert_global_value("fact".to_string(), fact);

        let clock = Value::Callable(Box::new(Clock {}));
        env.insert_global_value("klok".to_string(), clock);

        let fib = Value::Callable(Box::new(Fibonacci {}));
        env.insert_global_value("fib".to_string(), fib);

        Self { env }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> (bool, Value) {
        let mut error_found = false;
        let mut return_val = Value::Nil;

        for statement in statements {
            if let Err(e) = self.evaluate_stmt(&statement) {
                error_found = true;

                match e {
                    RuntimeErr::Err(line, msg) => rox_error(line, &msg),
                    RuntimeErr::Return { value } => {
                        rox_error(0, "Onverwachtte geef.");
                        return_val = value;
                    }
                }
            }
        }
        (error_found, return_val)
    }

    pub fn evaluate_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeErr> {
        match stmt {
            Stmt::Expr(expr) => {
                self.evaluate_expr(expr)?;
            }

            Stmt::Print(expr) => print!("{}", (self.evaluate_expr(expr)?).to_string()),
            Stmt::Println(expr) => println!("{}", (self.evaluate_expr(expr)?).to_string()),

            Stmt::Var { name, expr } => {
                let value = self.evaluate_expr(expr)?;
                self.env.insert_value(&name.lexeme, value);
            }

            Stmt::Block(statements) => self.evaluate_block_stmt(statements)?,

            Stmt::If {
                first_if,
                else_ifs,
                final_else,
            } => self.evaluate_if_stmt(first_if, else_ifs, final_else)?,

            Stmt::While { condition, body } => {
                while let Value::True = self.evaluate_expr(condition)? {
                    self.evaluate_stmt(body)?;
                }
            }

            Stmt::For {
                name,
                start,
                end,
                body,
            } => self.evaluate_for_stmt(name, start, end, body)?,

            Stmt::Function(funtion) => {
                let function = Value::Callable(Box::new(funtion.clone()));
                self.env.insert_value(&funtion.name.lexeme, function);
            }

            Stmt::Return { expr, .. } => {
                return Err(RuntimeErr::Return {
                    value: self.evaluate_expr(expr)?,
                });
            }
        }
        Ok(())
    }

    fn evaluate_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeErr> {
        self.env.create_new_child();
        for stmt in statements {
            if let Err(e) = self.evaluate_stmt(stmt) {
                self.env.kill_youngest_child();
                return Err(e);
            }
        }
        self.env.kill_youngest_child();
        Ok(())
    }

    fn evaluate_if_stmt(
        &mut self,
        first_if: &If,
        else_ifs: &Vec<If>,
        other: &Option<Box<Stmt>>,
    ) -> Result<(), RuntimeErr> {
        if let Value::True = self.evaluate_expr(&first_if.should_execute)? {
            // execute the first if
            self.evaluate_stmt(&first_if.statement)
        } else {
            // check for other else_ifs
            let mut else_if_executed = false;
            for else_if in else_ifs {
                if let Value::True = self.evaluate_expr(&else_if.should_execute)? {
                    self.evaluate_stmt(&else_if.statement)?;
                    else_if_executed = true;
                    break;
                }
            }

            // execute if there is an else and no else_ifs were executed
            if !else_if_executed {
                if let Some(other) = other {
                    self.evaluate_stmt(other)?
                }
            }
            Ok(())
        }
    }

    fn evaluate_for_stmt(
        &mut self,
        name: &Token,
        start: &Expr,
        end: &Expr,
        statement: &Stmt,
    ) -> Result<(), RuntimeErr> {
        let start_value = self.evaluate_expr(start)?;
        let end_value = self.evaluate_expr(end)?;

        if let (Value::Num(mut current), Value::Num(end)) = (start_value, end_value) {
            self.env.create_new_child();
            self.env.insert_value(&name.lexeme, Value::Num(current));

            while current < end {
                if let Err(e) = self.evaluate_stmt(statement) {
                    self.env.kill_youngest_child();
                    return Err(e);
                }

                current += 1.;
                if let Err(msg) = self.env.replace_value(name, &Value::Num(current)) {
                    self.env.kill_youngest_child();
                    return Err(RuntimeErr::Err(name.line, msg));
                }
            }

            while current > end {
                self.evaluate_stmt(statement)?;

                current -= 1.;
                if let Err(msg) = self.env.replace_value(name, &Value::Num(current)) {
                    self.env.kill_youngest_child();
                    return Err(RuntimeErr::Err(name.line, msg));
                }
            }
            self.env.kill_youngest_child();
            Ok(())
        } else {
            panic!("Unreachable.");
        }
    }

    pub fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeErr> {
        match expr {
            Expr::Lit(lit) => Ok(Value::from_lit(lit)),
            Expr::Grouping(expr) => self.evaluate_expr(expr),
            Expr::Unary(token, expr) => self.evaluate_unary_expr(token, expr),
            Expr::Binary(left, op, right) => self.evaluate_binary_expr(left, op, right),
            Expr::Var(token) => self.evaluate_var_expr(token),
            Expr::Assign(name, expr) => self.evaluate_assign_expr(name, expr),
            Expr::Logic(left, op, right) => self.evaluate_logic_expr(left, op, right),
            Expr::Call(callee, right_paren, args) => {
                self.evaluate_call_expr(callee, right_paren, args)
            }
        }
    }

    fn evaluate_call_expr(
        &mut self,
        callee: &Expr,
        right_paren: &Token,
        args: &Vec<Box<Expr>>,
    ) -> Result<Value, RuntimeErr> {
        let callee = self.evaluate_expr(callee)?;

        let mut arguments = Vec::new();
        for arg in args {
            arguments.push(self.evaluate_expr(arg)?);
        }

        if let Value::Callable(callee) = callee {
            if callee.arity() != arguments.len() {
                let msg = format!(
                    "Verwachtte {} argumenten maar kreeg er {}.",
                    callee.arity(),
                    arguments.len(),
                );
                return Err(RuntimeErr::Err(right_paren.line, msg));
            }
            callee.call(arguments, self)
        } else {
            let msg = "Je kan alleen functies en klassen bellen.".to_string();
            Err(RuntimeErr::Err(right_paren.line, msg))
        }
    }
    fn evaluate_unary_expr(
        &mut self,
        token: &Token,
        expr: &Box<Expr>,
    ) -> Result<Value, RuntimeErr> {
        let right = self.evaluate_expr(expr)?;

        match token.kind {
            TokenType::Minus => match right {
                Value::Num(num) => Ok(Value::Num(-num)),
                _ => Err(RuntimeErr::Err(
                    token.line,
                    "Min kan alleen worden gebruikt voor nummers.".to_string(),
                )),
            },
            TokenType::Bang => match right.is_true() {
                Some(bool) => Ok(Value::from_bool(!bool)),
                None => Err(RuntimeErr::Err(
                    token.line,
                    "Uitroepteken kan alleen worden gebruikt op waarheidswaardes.".to_string(),
                )),
            },
            _ => panic!("Unreachable."),
        }
    }

    fn evaluate_binary_expr(
        &mut self,
        left: &Box<Expr>,
        op: &Token,
        right: &Box<Expr>,
    ) -> Result<Value, RuntimeErr> {
        let left = self.evaluate_expr(left)?;
        let right = self.evaluate_expr(right)?;

        macro_rules! apply_arith_to_nums {
            ($type: ident, $op: tt) => {
                if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                    Ok(Value::Num(num1 $op num2))
                } else {
                    let msg = concat!(stringify!($op), " kan alleen worden gebruikt op nummers.");
                    Err(RuntimeErr::Err(op.line, msg.to_string()))
                }
            };
        }

        macro_rules! apply_logic_to_nums {
            ($type: ident, $op: tt) => {
                if let (Value::Num(num1), Value::Num(num2)) = (left, right) {
                    Ok(Value::from_bool(num1 $op num2))
                } else {
                    let msg = concat!(stringify!($op), " kan alleen worden gebruikt op nummers.");
                    Err(RuntimeErr::Err(op.line, msg.to_string()))
                }
            };
        }

        match op.kind {
            TokenType::Plus => match (left, right) {
                (Value::Num(num), Value::Str(str)) => {
                    return Ok(Value::Str(format!("{}{}", num, str)))
                }
                (Value::Str(str), Value::Num(num)) => {
                    return Ok(Value::Str(format!("{}{}", str, num)))
                }
                (Value::Num(num1), Value::Num(num2)) => return Ok(Value::Num(num1 + num2)),
                (Value::Str(str1), Value::Str(str2)) => {
                    return Ok(Value::Str(format!("{}{}", str1, str2)))
                }

                _ => Err(RuntimeErr::Err(
                    op.line,
                    "'+' kan alleen worden gebruikt op nummers en strings.".to_string(),
                )),
            },
            TokenType::Minus => apply_arith_to_nums!(Minus, -),
            TokenType::Star => apply_arith_to_nums!(Star, *),
            TokenType::Slash => apply_arith_to_nums!(Slash, /),

            TokenType::Caret => match (left, right) {
                (Value::Num(num1), Value::Num(num2)) => {
                    //if num2 < 0 {
                    //   num2 *= -1; 
                    //}
                    return Ok(Value::Num(num1.powf(num2)))},
                _ => Err(RuntimeErr::Err(
                    op.line,
                    "'^' kan alleen worden gebruikt op nummers.".to_string(),
                )),
            },

            TokenType::Greater => apply_logic_to_nums!(Greater, >),
            TokenType::GreaterEqual => apply_logic_to_nums!(GreaterEqual, >=),
            TokenType::Less => apply_logic_to_nums!(Less, <),
            TokenType::LessEqual => apply_logic_to_nums!(LessEqaul, <=),
            TokenType::Equal => apply_logic_to_nums!(Equal, ==),

            TokenType::EqualEqual => Ok(Value::from_bool(Value::is_equal(&left, &right))),
            TokenType::BangEqual => Ok(Value::from_bool(!Value::is_equal(&left, &right))),
            _ => panic!("Unreachable."),
        }
    }

    fn evaluate_logic_expr(
        &mut self,
        left: &Box<Expr>,
        op: &Token,
        right: &Expr,
    ) -> Result<Value, RuntimeErr> {
        match op.kind {
            TokenType::And => {
                let left = self.evaluate_expr(left)?.is_true();

                if let Some(left) = left {
                    let right = self.evaluate_expr(right)?.is_true();

                    if let Some(right) = right {
                        return Ok(Value::from_bool(left && right));
                    } else {
                        let msg =
                            "'en' kan alleen worden gebruikt op waardigheids waarden.".to_string();
                        Err(RuntimeErr::Err(op.line, msg))
                    }
                } else {
                    let msg =
                        "'en' kan alleen worden gebruikt op waardigheids waarden.".to_string();
                    Err(RuntimeErr::Err(op.line, msg))
                }
            }

            TokenType::Or => {
                match self.evaluate_expr(left)?.is_true() {
                    Some(left) => {
                        if left {
                            return Ok(Value::True);
                        }
                    }
                    None => {
                        let msg =
                            "'of' kan alleen worden gebruikt op waardigheids waarden.".to_string();
                        return Err(RuntimeErr::Err(op.line, msg));
                    }
                }

                match self.evaluate_expr(right)?.is_true() {
                    Some(right) => Ok(Value::from_bool(right)),
                    None => {
                        let msg =
                            "'of' kan alleen worden gebruikt op waardigheids waarden.".to_string();
                        return Err(RuntimeErr::Err(op.line, msg));
                    }
                }
            }
            _ => panic!("Unreachable."),
        }
    }

    fn evaluate_var_expr(&mut self, token: &Token) -> Result<Value, RuntimeErr> {
        match self.env.get_value(&token) {
            Some(value) => Ok(value),
            None => Err(RuntimeErr::Err(
                token.line,
                format!("'{}' is een onbekende variabele.", token.lexeme),
            )),
        }
    }

    fn evaluate_assign_expr(&mut self, name: &Token, expr: &Expr) -> Result<Value, RuntimeErr> {
        let new_value = self.evaluate_expr(expr)?;
        if let Err(msg) = self.env.replace_value(name, &new_value) {
            return Err(RuntimeErr::Err(name.line, msg));
        }
        Ok(new_value)
    }
}
