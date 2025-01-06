use crate::callable::FunDeclaration;
use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Println(Expr),
    Var {
        name: Token,
        expr: Expr,
    },
    Block(Vec<Stmt>),
    If {
        first_if: If,
        else_ifs: Vec<If>,
        final_else: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        name: Token,
        start: Expr,
        end: Expr,
        body: Box<Stmt>,
    },
    Function(FunDeclaration),
    Return {
        keyword: Token,
        expr: Expr,
    },
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
