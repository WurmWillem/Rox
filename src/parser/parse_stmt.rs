use crate::{
    callable::FunDeclaration,
    error::{rox_error, RoxError},
    expr::Expr,
    parser::Parser,
    stmt::{If, Stmt},
    token::Literal,
    token_type::TokenType,
};

impl Parser {
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

    pub fn declaration(&mut self) -> Result<Stmt, RoxError> {
        if self.matches(vec![TokenType::Var]) {
            match self.var_declaration() {
                Ok(stmt) => Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    Err(e)
                }
            }
        } else if self.matches(vec![TokenType::Fun]) {
            match self.fun_declaration("functie") {
                Ok(stmt) => Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    Err(e)
                }
            }
        } else {
            match self.statement() {
                Ok(stmt) => Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    Err(e)
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
}
