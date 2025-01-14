use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    None,
    Str(String),
    Num(i64),
    True,
    False,
    Nil,
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::None => write!(f, "None"),
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Num(n) => write!(f, "{}", n),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}
impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        //format!("{:?}{}{}", self.kind, self.lexeme, self.literal)
        match &self.literal {
            Literal::Str(s) => s.clone(),
            Literal::Num(n) => n.to_string(),
            _ => self.lexeme.clone(),
            /* _ => "".to_string(), */
        }
        //self.lexeme.clone()
    }
}
