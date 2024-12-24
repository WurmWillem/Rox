use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    None,
    Str(String),
    Num(f64),
    True,
    False,
    Nil,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    lexeme: String,
    pub literal: Literal,
    line: usize,
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
