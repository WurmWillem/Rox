use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Println(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
    If(If, Vec<If>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(Token, Expr, Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Box<Stmt>),
}

#[derive(Debug, Clone)]
pub struct If {
    pub should_execute: Expr,
    pub statement: Box<Stmt>,
}
impl If {
    pub fn new(should_execute: Expr, statement: Stmt) -> Self {
        If {
            should_execute,
            statement: Box::new(statement),
        }
    }
}
