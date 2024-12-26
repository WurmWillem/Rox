use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Expr),
}
