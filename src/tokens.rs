#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Function,
    FunctionCall(String),
    If,
    Then,
    Else,       
    Print,
    Plus,
    Eq,
    Minus,
    Multiply,
    Divide,
    Assign,
    LParen,
    RParen,
    Comma,
    Semicolon,
    End,
}

