use std::collections::HashMap;

use crate::error;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        //let mut keywords = HashMap::new();
        let keywords = HashMap::from([
            ("en".to_string(), TokenType::And),
            ("of".to_string(), TokenType::Or),
            ("als".to_string(), TokenType::If),
            ("anders".to_string(), TokenType::Else),
            ("terwijl".to_string(), TokenType::While),
            ("voor".to_string(), TokenType::For),
            ("wellus".to_string(), TokenType::True),
            ("nietus".to_string(), TokenType::False),
            ("niks".to_string(), TokenType::Nil),
            ("dit".to_string(), TokenType::This),
            ("ouder".to_string(), TokenType::Super),
            ("klas".to_string(), TokenType::Class),
            ("fun".to_string(), TokenType::Fun),
            ("laat".to_string(), TokenType::Var),
            ("retour".to_string(), TokenType::Return),
            ("yap".to_string(), TokenType::Print),
        ]);

        Self {
            source,
            tokens: vec![],
            keywords,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end_input() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::None,
            self.line,
        ));

        self.tokens.clone()
    }

    fn at_end_input(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.get_current_char();
        self.current += 1;

        macro_rules! ternary {
            ($c: expr, $t1: ident, $t2: ident) => {{
                let token = if self.same($c) {
                    self.current += 1;
                    TokenType::$t1
                } else {
                    TokenType::$t2
                };
                self.add_token(token);
            }};
        }

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => ternary!('=', BangEqual, Bang),
            '=' => ternary!('=', EqualEqual, Equal),
            '<' => ternary!('=', LessEqual, Less),
            '>' => ternary!('=', GreaterEqual, Greater),

            // comments
            '/' => {
                if self.same('/') {
                    while self.peek() != '\n' && !self.at_end_input() {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash);
                    self.current += 1;
                }
            }

            '"' => {
                while self.peek() != '"' && !self.at_end_input() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.current += 1;
                }
                if self.at_end_input() {
                    error(self.line, "Unterminated string, bozo");
                    return;
                }

                self.current += 1;

                let lit =
                    Literal::Str(self.source[(self.start + 1)..(self.current - 1)].to_string());

                self.add_lit_token(TokenType::String, lit);
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                if c.is_digit(10) {
                    self.add_num_token()
                } else if c.is_alphabetic() || c == '_' {
                    while self.peek().is_alphanumeric() {
                        self.current += 1;
                    }
                    self.add_token(TokenType::Identifier)
                } else {
                    error(self.line, "unexpected character, seems like a skill issue");
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.at_end_input() {
            '\0'
        } else {
            self.get_current_char()
        }
    }

    fn same(&mut self, expected: char) -> bool {
        !self.at_end_input() && self.get_current_char() == expected
    }

    fn get_current_char(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    fn get_next_char(&self) -> char {
        self.source.as_bytes()[self.current + 1] as char
    }

    fn add_lit_token(&mut self, kind: TokenType, lit: Literal) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(kind, text, lit, self.line));
    }
    fn add_token(&mut self, kind: TokenType) {
        self.add_lit_token(kind, Literal::None)
    }

    fn add_num_token(&mut self) {
        while self.peek().is_digit(10) {
            self.current += 1;
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.current += 1;

            while self.peek().is_digit(10) {
                self.current += 1;
            }
        }

        let num = self.source[(self.start + 1)..(self.current - 1)].to_string();
        let num = num.parse::<f64>().unwrap();
        self.add_lit_token(TokenType::Number, Literal::Num(num))
    }

    fn peek_next(&self) -> char {
        if self.current >= self.source.len() {
            '\0'
        } else {
            self.get_next_char()
        }
    }
}