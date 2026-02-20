// abstract syntax tree

#[derive(Debug, Clone)]
pub enum Statement {
    Assign{
        name: String,
        value: Expression,
    },
    Print(Expression),
    Send(Expression),
    Function{
        name: String,
        params: Vec<Expression>,
        body: Vec<Statement>,
    },
    FunctionCall{
        name: String,
        args: Vec<Expression>
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Fetch {
        method: Expression,
        url: Expression,
        body: Option<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    StringLiteral(String),
    FunctionArg(String),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    FunctionCall{
        name: String,
        args: Vec<Expression>
    },
    Fetch {
        method: Box<Expression>,
        url: Box<Expression>,
        body: Option<Box<Expression>>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NEq,
    Gt,
    Lt,
    LtEq,
}


