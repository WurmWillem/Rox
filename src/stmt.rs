use crate::expr::Expr;
use crate::interpreter;

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}
impl Stmt {
    pub fn evaluate(&self) {
        match self {
            Stmt::Expr(expr) => {
                interpreter::evaluate(expr);
            },
            Stmt::Print(expr) => {
                println!("{:?}", interpreter::evaluate(expr).to_string());
            }
        }
    }
}
