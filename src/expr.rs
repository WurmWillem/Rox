use crate::token::{Literal, Token};

#[derive(Clone)]
pub enum Expr {
    Lit(Literal),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}
impl Expr {
    pub fn stringify(&self) -> String {
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
            Expr::Binary(left, token, right) => {
                let left = *left.clone();
                let right = *right.clone();
                parenthesize(token.lexeme.clone(), vec![left, right])
            }
        }
    }
}
fn parenthesize(name: String, exprs: Vec<Expr>) -> String {
    let mut out = format!("({}", name.clone());

    for i in 0..exprs.len() {
        out.push_str(" ");
        out.push_str(&exprs[i].stringify());
    }

    out.push_str(")");
    out
}
