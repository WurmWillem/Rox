use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Println(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Vec<(Expr, Box<Stmt>)>, Option<Box<Stmt>>),
}
