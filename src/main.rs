use compiler::Compiler;

use crate::lexer::Lexer;
use crate::parser::parse_program;
use std::fs;
use std::io::Write;
use std::fs::File;
mod ast;
mod compiler;
mod lexer;
mod parser;
mod tokens;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.bonk> <output.asm>", args[0]);
        std::process::exit(1);
    }

    let file_input = fs::read_to_string(&args[1])
        .expect("Error reading input file");

    let mut lexer = Lexer::new(file_input);
    let tokens = lexer.tokenise();

    let ast = parse_program(&tokens).expect("Parsing error");

    let mut comp = Compiler::new();
    let result = comp.compile(ast);

    let output_path = &args[2];
    let mut file = File::create(output_path).expect("Unable to create output file");

    for line in result {
        writeln!(file, "{}", line).expect("Failed to write line");
    }

    println!("Assembly written to {}", output_path);
}
