use std::io::{self, Write};

use console::Term;    
mod tokens; 
mod lexer;
mod parser;

fn read_input() {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut lexer = lexer::Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    parser::parse(&tokens);


}
fn main() {

    let term = Term::stdout();
    loop {
        read_input(); 
    }

}
