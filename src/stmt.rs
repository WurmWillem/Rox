use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Println(Expr),
    Var(Token, Expr),
}
