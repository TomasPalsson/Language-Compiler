#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Function,
    FunctionCall(String),
    If,
    While,
    Do,
    Then,
    Else,       
    Print,
    Plus,
    Eq,
    NotEq,
    Greater,
    Less,
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

