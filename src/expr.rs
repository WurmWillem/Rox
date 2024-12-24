use crate::token::{Literal, Token};

#[derive(Clone)]
pub enum Expr {
    Lit(Literal),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}
