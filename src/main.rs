use std::fs;

use token_type::TokenType;

mod token_type;

fn main() {
    let mut lox = Lox::new();
    lox.run("file.txt".to_string());
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }
    fn run(&mut self, mut source: String) {
        source = fs::read_to_string(source).unwrap();
        //for i in 0..source.len() {
        //    if source.as_bytes()[i] as char == '\n' {
        //        print!("s");
        //    }
        //
        //    //print!("{}", source.as_bytes()[i] as char);
        //}
        //return;
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();
        for token in tokens {
            print!("{}", token.to_string());
        }
    }
}
fn error(line: usize, message: &str) {
    panic!("[line {}] Error: {}", line, message);
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end_input() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

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
                self.add_token(TokenType::String);
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                if c.is_digit(10) {
                    self.add_num_token()
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

    fn add_token(&mut self, kind: TokenType) {
        let text = (&self.source[self.start..self.current]).to_string();

        let mut literal = None;
        if kind == TokenType::String {
            literal = Some((&self.source[(self.start + 1)..(self.current - 1)]).to_string());
        }

        self.tokens.push(Token::new(kind, text, literal, self.line));
    }

    fn add_num_token(&self) {
        todo!()
    }
}

fn is_digit(c: char) -> bool {
    todo!()
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn new(kind: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
    fn to_string(&self) -> String {
        //format!("{:?}{}{}", self.kind, self.lexeme, self.literal)
        match &self.literal {
            Some(l) => l.clone(),
            _ => self.lexeme.clone(),
        }
    }
}
