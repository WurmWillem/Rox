use super::Parser;

use crate::{error::RoxError, expr::Expr, token::Literal, token_type::TokenType};

impl Parser {
    pub fn parse_expr(&mut self) -> Result<Expr, RoxError> {
        self.expression()
    }

    pub fn expression(&mut self) -> Result<Expr, RoxError> {
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
                        msg: "Hier kan je niet aan assignen.".to_string(),
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
        let mut expr = self.list()?;

        while self.matches(vec![TokenType::Caret]) {
            let op = self.previous();
            let right = self.list()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn list(&mut self) -> Result<Expr, RoxError> {
        let mut elements = Vec::new();

        if self.matches(vec![TokenType::LeftBracket]) {
            elements.push(self.expression()?);
            while self.matches(vec![TokenType::Comma]) {
                elements.push(self.expression()?);
            }
            self.consume(TokenType::RightBracket, "Verwachtte ']' na elementen")?;

            //print!("[");
            //for element in &elements {
            //    print!("{:?}, ", element);
            //}
            //println!("]");

            return Ok(Expr::List(elements));
        }

        self.index()
    }

    fn index(&mut self) -> Result<Expr, RoxError> {
        let mut var = self.call()?;

        if self.matches(vec![TokenType::LeftBracket]) {
            let index = self.expression()?;
            let right_bracket = self.consume(TokenType::RightBracket, "Verwachtte ']' na index")?;

            var = Expr::Index {
                var: Box::new(var),
                index: Box::new(index),
                right_bracket,
            };
        }

        Ok(var)
    }

    fn call(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
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
}
