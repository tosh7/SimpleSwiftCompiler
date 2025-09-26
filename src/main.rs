mod compiler;
mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;
mod llvm_backend;
mod options;

use std::fs;
use std::env;
use std::process;

use crate::options::CompilerOption;
use crate::compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <source-file>", args[0]);
        process::exit(1);
    }

    let mut filename= None;
    let mut options: Vec<CompilerOption> = Vec::new();
    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            // verbose option on
            if let Some(option) = CompilerOption::from_literal(&arg) {
                options.push(option);
            }
        } else {
            filename = Some(arg);
        }
    }

    print!("{:?}", options);

    let Some(filename) = filename else {
        eprintln!("Usage: {} <source-file>", args[0]);
        process::exit(1);
    };
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file {}: {}", filename, error);
            process::exit(1);
        }
    };
    println!("Source code:\n{}", source);

    let compiler = Compiler::new();
    if let Err(e) = compiler.compile(&source) {
        eprintln!("Compilation error: {}", e);
        process::exit(1);
    }
}