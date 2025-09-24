mod compiler;
mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;
mod llvm_backend;

use std::fs;
use std::env;
use std::process;

use crate::compiler::Compiler;

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

    let compiler = Compiler::new();
    compiler.compile(&source);
}