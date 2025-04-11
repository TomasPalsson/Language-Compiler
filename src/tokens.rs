#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Function,
    EndFunction,
    FunctionCall(String),
    Print,
    Plus,
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

