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
            if c.is_alphanumeric() || c == '~' {
                self.advance();
            } else {
                break;
            }
        }
        let identifier = &self.input[start..self.position];
        if identifier.strip_prefix('~').is_some() {
            return Token::FunctionCall(identifier[1..].to_string());
        }
        match identifier {
            "run" => Token::Function,
            "end" => Token::End,
            "while" => Token::While,
            "do" => Token::Do,
            "print" => Token::Print,
            "send" => Token::Send,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            _ => Token::Identifier(identifier.to_string()),
        }
    }
    fn lex_operator(&mut self) -> Token {
        let c = self.peek().unwrap();
        if matches!(c, '=') {
            self.advance();
            if self.peek() == Some('=') {
                self.advance();
                return Token::Eq;
            } else {
                return Token::Assign;
            }
        }
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '<' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LessEq
                } else {
                    Token::Less
                }
            }
            '>' => Token::Greater,
            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::NotEq
                } else {
                    panic!("Unexpected character after '!': {}", self.peek().unwrap()); 
                }
            }
            ';' => Token::Semicolon,
            '"' => {
                self.advance();
                let start = self.position;
                while let Some(c) = self.peek() {
                    if c == '"' {
                        self.advance();
                        break;
                    } else {
                        self.advance();
                    }
                }
                Token::StringLiteral(self.input[start..self.position - 1].to_string())
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }
    pub fn tokenise(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.skip_whitespace();
            } else if c.is_ascii_digit() {
                tokens.push(Token::Number(self.lex_number()));
            } else if c.is_alphabetic() || c == '~' {
                tokens.push(self.lex_identifier());
            } else {
                let tok = self.lex_operator();
                tokens.push(tok.clone());
                match tok {
                    Token::Eq | Token::NotEq | Token::Less | Token::LessEq | Token::StringLiteral(_) => {}
                    _ => self.advance(),
                }
            }
        }
        tokens
    }
}
