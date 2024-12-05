use crate::token::{Literal, Token};

pub enum Expr {
    Lit(Literal),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}
