use crate::tokens::Token;
use std::{self, iter::Peekable};
use std::slice::Iter;
use crate::ast::{BinaryOperator, Expression, Statement};

pub fn parse_program(tokens: &[Token]) -> Option<Vec<Statement>>{
    let mut iter = tokens.iter().peekable();
    let mut statements = Vec::new();

    while iter.peek().is_some() {
        let stmt = parse_statement(&mut iter);
        statements.push(stmt.expect("Unexpected Statement"));
    }

    Some(statements)
}


fn parse_statement(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    
    let statement = match iter.peek() {
        Some(Token::Function) => parse_function(iter),
        Some(Token::FunctionCall(name)) => {
            let name = name.clone();
            iter.next();
            let mut args = Vec::new();
            if matches!(iter.peek(), Some(Token::LParen)) {
                iter.next();
                while !matches!(iter.peek(), Some(Token::RParen)) {
                    args.push(parse_expression(iter)?);
                    if matches!(iter.peek(), Some(Token::Comma)) {
                        iter.next();
                    }
                }
                iter.next(); // consume RParen
            }
            Ok(Statement::FunctionCall{ name, args })
        },
        Some(Token::If) => parse_if(iter),

        Some(Token::Print) => {
            iter.next();
            let expr = parse_expression(iter)?;
            Ok(Statement::Print(expr))
        }
        Some(Token::Send) => {
            iter.next();
            let expr = parse_expression(iter)?;
            Ok(Statement::Send(expr))
        }
        Some(Token::While) => parse_while(iter),
        Some(Token::Identifier(_)) => parse_assignment(iter),
        _ => Err(format!("Cannot parse found {:?}", iter.peek())),
    }?;

    if let Some(Token::Function) = iter.peek() {
        return Ok(statement);
    }

    let next_token = iter.peek();
    match next_token {
        Some(Token::End) => {
            iter.next();
            Ok(statement)
        }
        Some(Token::Semicolon) => {
            iter.next();
            Ok(statement)
        }
        Some(Token::Then) => {
            iter.next();
            Ok(statement)
        }
        Some(tok) => Err(format!("Expected semicolon, got {:?}", tok)),
        None => Ok(statement),
    }
}

fn parse_function(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    iter.next();
    let name = match iter.peek() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => "Name".to_string(),
    };
    iter.next();
    let mut args = Vec::new();
    if matches!(iter.peek(), Some(Token::LParen)) {
        iter.next();
        while !matches!(**iter.peek().unwrap(), Token::RParen) {
            let arg = parse_arg(iter);
            if let Ok(arg) = &arg {
                args.push(arg.clone());
            } else if let Err(err) = &arg {
                panic!("{}", err);
            }

            if matches!(**iter.peek().unwrap(), Token::Comma) {
                iter.next();
            } else if !matches!(iter.peek().unwrap(), Token::RParen) {
                return Err("Function Setup incorrectly".to_string())
            }
        }
        iter.next();
    }


    let mut body = Vec::new();
    while !matches!(**iter.peek().unwrap(), Token::End) {
        let statement = parse_statement(iter);
        body.push(statement?);
    }
    Ok(Statement::Function{
        name,
        params: args,
        body
    })
}

fn parse_if(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    iter.next(); // Consuming if
    let condition = parse_expression(iter)?;
    expect_token(iter, Token::Then)?;
    iter.next();

    let mut then_body = Vec::new();
    while !matches!(iter.peek(), Some(Token::Else) | Some(Token::End)) {
        then_body.push(parse_statement(iter)?);
    }

    let mut else_body = None;
    if matches!(iter.peek(), Some(Token::Else)) {
        iter.next(); // Consuming else
        else_body = Some(Vec::new());
        while !matches!(iter.peek(), Some(Token::End)) {
            else_body.as_mut().unwrap().push(parse_statement(iter)?);
        }
    }
    
    expect_token(iter, Token::End)?;
    Ok(Statement::If {
        condition,
        then_body,
        else_body,
    })
}

fn parse_arg(iter: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    match iter.peek().unwrap() {
        Token::Identifier(na) => {
            let name = na.clone();
            iter.next();
            Ok(Expression::FunctionArg(name))
        },
        _ => Err("Arg defined incorrectly".to_string()),
    }

}

fn parse_assignment(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    let name_token = iter.next().ok_or("Expected identifier")?;
    let name = match name_token {
        Token::Identifier(n) => n.clone(),
        _ => return Err("Expected identifier".into()),
    };

    match iter.next() {
        Some(Token::Assign) => {}
        _ => return Err("Expected '=' after identifier".into()),
    };

    let value = parse_expression(iter)?;

    Ok(Statement::Assign{ name, value })
}

fn get_operator(token: &Token) -> Result<BinaryOperator, String> {
    match token {
        Token::Plus => Ok(BinaryOperator::Add),
        Token::Minus => Ok(BinaryOperator::Sub),
        Token::Multiply => Ok(BinaryOperator::Mul),
        Token::Divide => Ok(BinaryOperator::Div),
        Token::Eq => Ok(BinaryOperator::Eq),
        Token::NotEq => Ok(BinaryOperator::NEq),
        Token::Less => Ok(BinaryOperator::Lt),
        Token::LessEq => Ok(BinaryOperator::LtEq),
        Token::Greater => Ok(BinaryOperator::Gt),
        _ => Err(format!("Error in parsing operator: {:?}", token)),
    }

}

fn expect_token(iter: &mut Peekable<Iter<Token>>, expected: Token) -> Result<(), String> {
    if let Some(token) = iter.peek() {
        if **token == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, token))
        }
    } else {
        Err(format!("Expected {:?}, but reached end of input", expected))
    }
}

fn get_precedence(op: &Token) -> u8 {
    match op {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

fn parse_expression(iter: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    parse_binary_expression(iter, 0) 
}

fn parse_binary_expression(iter: &mut Peekable<Iter<Token>>, min_prec: u8) -> Result<Expression, String> {
    let mut left = parse_atomics(iter)?;

    while let Some(op_token) = iter.peek() {
        if !matches!(op_token, Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Eq | Token::Less | Token::LessEq | Token::Greater | Token::NotEq) {
            break;
        }
        let prec = get_precedence(op_token);
        if prec < min_prec {
            break;
        }
        let op = get_operator(op_token)?;
        iter.next(); 
        let right = parse_binary_expression(iter, prec + 1)?;
        left = Expression::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    } 
    Ok(left)
}


fn parse_atomics(iter: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    match iter.peek() {
        Some(Token::Number(n)) => {
            iter.next();
            Ok(Expression::Integer(*n))
        }
        Some(Token::StringLiteral(s)) => {
            iter.next();
            Ok(Expression::StringLiteral(s.clone()))
        }
        Some(Token::Identifier(name)) => {
            iter.next();
            Ok(Expression::Variable(name.clone()))
        }
        Some(Token::FunctionCall(_)) => {
            let name = match iter.next() {
                Some(Token::FunctionCall(n)) => n.clone(),
                _ => unreachable!(),
            };
            let mut args = Vec::new();
            if matches!(iter.peek(), Some(Token::LParen)) {
                iter.next();
                while !matches!(iter.peek(), Some(Token::RParen)) {
                    args.push(parse_expression(iter)?);
                    if matches!(iter.peek(), Some(Token::Comma)) {
                        iter.next();
                    }
                }
                iter.next(); // consume RParen
            }
            Ok(Expression::FunctionCall { name, args })
        }
        other => Err(format!("Expected expression, got {:?}", other)),
    }
}

fn parse_while(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    iter.next(); // Consuming while
    let condition = parse_expression(iter)?;
    expect_token(iter, Token::Do)?;
    iter.next();

    let mut body = Vec::new();
    while !matches!(iter.peek(), Some(Token::End)) {
        body.push(parse_statement(iter)?);
    }
    
    expect_token(iter, Token::End)?;
    Ok(Statement::While {
        condition,
        body,
    })
}


