mod lexer;
mod token;
mod ast;
mod parser;

use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <source-file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file {}: {}", filename, error);
            process::exit(1);
        }
    };
    println!("Source code:\n{}", source);

    let tokens = match lexer::tokenize(&source) {
        Ok(tokens) => {
            println!("Tokens:");
            for(i, token) in tokens.iter().enumerate() {
                println!("{}: {:?}", i, token);
            }
            tokens
        }
        Err(error) => {
            eprintln!("Lexing error: {}", error);
            return;
        }
    };

    match parser::parse(tokens) {
        Ok(ast) => {
            println!("=== AST ===");
            println!("{:#?}", ast);
        }
        Err(error) => {
            eprintln!("AST error: {}", error);
        }
    }
}