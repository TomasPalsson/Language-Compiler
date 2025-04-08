use crate::tokens::Token;

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
        }
    }
    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position..].chars().next().unwrap())
        } else {
            None
        }
    }
    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn lex_number(&mut self) -> i64 {
        let start = self.position;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].parse().unwrap_or(0)
    }
    fn lex_identifier(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let identifier = &self.input[start..self.position];
        match identifier {
            "ab" => Token::Function,
            "endf" => Token::EndFunction,
            _ => Token::Identifier(identifier.to_string()),
        }
    }
    fn lex_operator(&mut self) -> Token {
        let c = self.peek().unwrap();
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '=' => Token::Assign,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '\n' => Token::End,
            _ => panic!("Unexpected character: {}", c),
        }
    }
    pub fn tokenise(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.skip_whitespace();
            } else if c.is_digit(10) {
                tokens.push(Token::Number(self.lex_number()));
            } else if c.is_alphabetic() {
                tokens.push(self.lex_identifier());
            } else {
                tokens.push(self.lex_operator());
                self.advance();
            }
        }
        tokens
    }
}
