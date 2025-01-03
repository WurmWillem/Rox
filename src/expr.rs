use crate::token::{Literal, Token};

#[derive(Clone)]
pub enum Expr {
    Lit(Literal),
    Logic(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Var(Token),
    Assign(Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Box<Expr>>),
}
// used for debugging purposes
impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Lit(lit) => lit.to_string(),
            Expr::Grouping(expr) => {
                let expr = *expr.clone();
                parenthesize("group".to_owned(), vec![expr])
            }
            Expr::Unary(token, expr) => {
                let expr = *expr.clone();
                parenthesize(token.lexeme.clone(), vec![expr])
            }
            Expr::Binary(left, token, right) | Expr::Logic(left, token, right) => {
                let left = *left.clone();
                let right = *right.clone();
                parenthesize(token.lexeme.clone(), vec![left, right])
            }
            Expr::Assign(_, _) => panic!("Unreachable."),
            Expr::Var(_) => panic!("Unreachable."),
            Expr::Call(_, _, _) => panic!("Unreachable."),
        }
    }
}

fn parenthesize(name: String, exprs: Vec<Expr>) -> String {
    let mut out = format!("({}", name.clone());

    for i in 0..exprs.len() {
        out.push_str(" ");
        out.push_str(&exprs[i].to_string());
    }

    out.push_str(")");
    out
}
