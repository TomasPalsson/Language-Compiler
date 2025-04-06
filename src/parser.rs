use crate::tokens::Token;
use std::{self, iter::Peekable};
use std::slice::Iter;

pub fn parse(
    tokens: &[Token],
) -> Result<String, String> {
    println!("Parsing tokens: {:?}", tokens);
    let mut iter = tokens.iter().peekable();
    parse_expression(&mut iter);
    Ok(String::from("Successful"))
}


fn parse_expression(iter: &mut Peekable<Iter<Token>>) -> Result<String, String> {
    let first = iter.peek();
    match first{
        Some(Token::Identifier(_)) => assignment(iter),
        _ => Err(String::from("No Token Found!"))
        
    }
}

fn assignment(iter: &mut Peekable<Iter<Token>>) -> Result<String, String> {
    let name = iter.next();
    if name.is_none() {
        println!("You need to assign something!");
        return Err(String::from("Oops"));
    }
    let name = name.unwrap();
    if iter.peek().is_none() {
        if let Token::Identifier(word) = name {
            println!("Assigning identifier with name: {:?}", word);
        }
        return Ok(String::from("Word!"));
    }
    if (**iter.peek().unwrap()) == Token::Assign {
        iter.next();
        let assignee = iter.next();
        if assignee.is_none() {
            println!("You need to assign something!");
            return Err(String::from("Oops"));
        } 
        let assignee = assignee.unwrap();
        Ok(String::from("Word!"))
    }
    else {
        Err(String::from("OOPS"))
    }

}
