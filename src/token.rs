use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
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
            Some(l) => l.clone(),
            _ => self.lexeme.clone(),
        }
    }
}
