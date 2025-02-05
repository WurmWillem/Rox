use crate::{
    callable::FunDeclaration,
    error::{rox_error, RoxError},
    expr::Expr,
    stmt::{If, Stmt},
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, RoxError> {
        self.expression()
    }

    pub fn parse_statements(&mut self) -> Option<Vec<Stmt>> {
        let mut parse_error_found = false;
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(declaration) => statements.push(declaration),
                Err(e) => {
                    match e {
                        RoxError::ParseError { line, msg } => rox_error(line, &msg),
                        _ => panic!("Unreachable."),
                    }
                    parse_error_found = true;
                }
            }
        }

        if parse_error_found {
            None
        } else {
            Some(statements)
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Println
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn declaration(&mut self) -> Result<Stmt, RoxError> {
        if self.matches(vec![TokenType::Var]) {
            match self.var_declaration() {
                Ok(stmt) => return Ok(stmt),
                Err(e) => {
                    self.synchronize();

                    return Err(e);
                }
            }
        } else if self.matches(vec![TokenType::Fun]) {
            match self.fun_declaration("functie") {
                Ok(stmt) => return Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    return Err(e);
                }
            }
        } else {
            match self.statement() {
                Ok(stmt) => return Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    return Err(e);
                }
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, RoxError> {
        let name = self.consume(
            TokenType::Identifier,
            "Je moet wel een naam aan de variabele geven.",
        )?;

        let mut value = Expr::Lit(Literal::Nil);
        if self.matches(vec![TokenType::Equal]) {
            value = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Je bent de ';' vergeten.")?;
        Ok(Stmt::Var { name, expr: value })
    }

    fn fun_declaration(&mut self, kind: &str) -> Result<Stmt, RoxError> {
        let msg = format!("Je moet wel een naam aan de {} geven", kind);
        let name = self.consume(TokenType::Identifier, &msg)?;

        let msg = format!("Verwachtte '(' na de {} naam.", kind);
        self.consume(TokenType::LeftParen, &msg)?;

        let mut params = Vec::new();
        if !self.matches(vec![TokenType::RightParen]) {
            params.push(self.consume(TokenType::Identifier, "Verwachtte parameter na comma.")?);

            while self.matches(vec![TokenType::Comma]) {
                params.push(self.consume(TokenType::Identifier, "Verwachtte parameter na comma.")?)
            }
            self.consume(TokenType::RightParen, "Verwachtte ')' na parameter.")?;
        }

        let msg = format!("Verwachtte '{{' na de {} naam.", kind);
        self.consume(TokenType::LeftBrace, &msg)?;

        let body = match self.block_statement()? {
            Stmt::Block(statements) => statements,
            _ => panic!("Unreachable."),
        };
        //for stmt in &body  {
        //dbg!(stmt);
        //}

        Ok(Stmt::Function(FunDeclaration { name, params, body }))
    }

    fn statement(&mut self) -> Result<Stmt, RoxError> {
        if self.matches(vec![TokenType::Print]) {
            return self.print_statement();
        } else if self.matches(vec![TokenType::Println]) {
            return self.println_statement();
        } else if self.matches(vec![TokenType::LeftBrace]) {
            return self.block_statement();
        } else if self.matches(vec![TokenType::If]) {
            return self.if_statement();
        } else if self.matches(vec![TokenType::While]) {
            return self.while_statement();
        } else if self.matches(vec![TokenType::For]) {
            return self.for_statement();
        } else if self.matches(vec![TokenType::Return]) {
            return self.return_statement();
        }
        self.expr_statement()
    }

    fn return_statement(&mut self) -> Result<Stmt, RoxError> {
        let keyword = self.previous();

        let mut expr = Expr::Lit(Literal::Nil);
        if let Ok(new_expr) = self.expression() {
            expr = new_expr;
        }

        self.consume(TokenType::Semicolon, "verwachtte ';' na geef statement.")?;
        Ok(Stmt::Return { keyword, expr })
    }

    fn block_statement(&mut self) -> Result<Stmt, RoxError> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "je bent een '}' vergeten druiloor")?;
        Ok(Stmt::Block(statements))
    }

    fn if_statement(&mut self) -> Result<Stmt, RoxError> {
        let first_if = If::new(self.expression()?, self.statement()?);

        let mut else_ifs = Vec::new();

        let mut final_else = None;
        while self.matches(vec![TokenType::Else]) {
            if self.matches(vec![TokenType::If]) {
                let else_if = If::new(self.expression()?, self.statement()?);
                else_ifs.push(else_if);
            } else {
                final_else = Some(Box::new(self.statement()?));
                break;
            }
        }

        Ok(Stmt::If {
            first_if,
            else_ifs,
            final_else,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, RoxError> {
        let condition = self.expression()?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, RoxError> {
        let name = self.consume(
            TokenType::Identifier,
            "Je moet wel een naam aan de variabele geven.",
        )?;
        self.consume(TokenType::From, "Verwachtte 'van'.")?;

        let start = self.expression()?;
        self.consume(TokenType::Until, "Verwachtte 'tot'.")?;
        let end = self.expression()?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::For {
            name,
            start,
            end,
            body,
        })
    }

    fn print_statement(&mut self) -> Result<Stmt, RoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Je bent een ';' vergeten druiloor")?;
        Ok(Stmt::Print(expr))
    }

    fn println_statement(&mut self) -> Result<Stmt, RoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Je bent een ';' vergeten druiloor")?;
        Ok(Stmt::Println(expr))
    }

    fn expr_statement(&mut self) -> Result<Stmt, RoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Je bent een ';' vergeten druiloor")?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, RoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, RoxError> {
        let expr = self.or()?;

        if self.matches(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Var(name) => return Ok(Expr::Assign(name, Box::new(value))),
                _ => {
                    let err = RoxError::ParseError {
                        line: equals.line,
                        msg: "Dit kan je niet assignen.".to_string(),
                    };
                    return Err(err);
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, RoxError> {
        let left = self.and()?;

        while self.matches(vec![TokenType::Or]) {
            let op = self.previous();
            let right = self.and()?;
            return Ok(Expr::Logic(Box::new(left), op, Box::new(right)));
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expr, RoxError> {
        let left = self.equality()?;

        while self.matches(vec![TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            return Ok(Expr::Logic(Box::new(left), op, Box::new(right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.comparison()?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.term()?;

        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.factor()?;

        while self.matches(vec![TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.unary()?;

        while self.matches(vec![TokenType::Star, TokenType::Slash]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RoxError> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.power()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }

        self.power()
    }

    fn power(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.call()?;

        while self.matches(vec![TokenType::Caret]) {
            let op = self.previous();
            let right = self.call()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn call(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.list()?;

        loop {
            if self.matches(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr.clone())?;
            } else {
                break;
            }
        }

        Ok(expr)
    }
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, RoxError> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            arguments.push(self.expression()?);
            while self.matches(vec![TokenType::Comma]) {
                arguments.push(self.expression()?);
            }
        }

        let token = self.consume(TokenType::RightParen, "Verwachtte ')' na argumenten")?;

        Ok(Expr::Call(Box::new(callee), token, arguments))
    }

    fn list(&mut self) -> Result<Expr, RoxError> {
        let mut elements = Vec::new();

        if self.matches(vec![TokenType::LeftBracket]) {
            elements.push(self.primary()?);
            while self.matches(vec![TokenType::Comma]) {
                elements.push(self.primary()?);
            }
            self.consume(TokenType::RightBracket, "Verwachtte ']' na elementen")?;

            print!("[");
            for element in &elements {
                print!("{:?}, ", element);
            }
            println!("]");

            return Ok(Expr::List(elements));
        }

        //if !self.check(TokenType::) {
        //    elements.push(self.expression()?);
        //    while self.matches(vec![TokenType::Comma]) {
        //        elements.push(self.expression()?);
        //    }
        //}
        //
        //let token = self.consume(TokenType::RightParen, "Verwachtte ')' na argumenten")?;

        self.primary()
        //Ok(Expr::Call(Box::new(callee), token, elements))
    }

    fn primary(&mut self) -> Result<Expr, RoxError> {
        if self.matches(vec![TokenType::True]) {
            return Ok(Expr::Lit(Literal::True));
        } else if self.matches(vec![TokenType::False]) {
            return Ok(Expr::Lit(Literal::False));
        } else if self.matches(vec![TokenType::Nil]) {
            return Ok(Expr::Lit(Literal::Nil));
        }

        if self.matches(vec![TokenType::Identifier]) {
            return Ok(Expr::Var(self.previous()));
        }

        if self.matches(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Lit(self.previous().literal));
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Je bent de ')' vergeten (je mag niet meer op mijn kinderfeestje komen)",
            )?;

            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let msg = format!(
            "Verwachtte een expressie. {:?} past hier niet.",
            self.peek().kind
        );
        Err(RoxError::ParseError {
            line: self.peek().line,
            msg,
        })
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, RoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(RoxError::ParseError {
                line: self.previous().line,
                msg: msg.to_string(),
            })
        }
    }

    fn matches(&mut self, t: Vec<TokenType>) -> bool {
        for i in 0..t.len() {
            if self.check(t[i]) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
