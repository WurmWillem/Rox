use crate::{
    expr::Expr,
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: vec![],
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.same(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.same(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.same(vec![TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.same(vec![TokenType::Star, TokenType::Slash]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.same(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary(op, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.same(vec![TokenType::True]) {
            return Expr::Lit(Literal::True);
        }
        if self.same(vec![TokenType::False]) {
            return Expr::Lit(Literal::False);
        }
        if self.same(vec![TokenType::Nil]) {
            return Expr::Lit(Literal::Nil);
        }

        if self.same(vec![TokenType::Number, TokenType::String]) {
            return Expr::Lit(self.previous().literal);
        }

        if self.same(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            return Expr::Grouping(Box::new(expr));
        }

        panic!("unreachable");
    }

    fn same(&mut self, t: Vec<TokenType>) -> bool {
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
