#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Function,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    LParen,
    RParen,
    End,
}

