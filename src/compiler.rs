mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;
mod llvm_backend;

use std::process::Command;

pub struct Compiler {
    backend: LLVMCompiler,
}

impl Compiler {
    pub fn new(input: String) -> Self {
        Compiler {
            backend
        }
    }

    pub fn compile(&self, source: &str) -> Result<(), String> {
        // frontend
            // toke
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
            // ast
            let ast = match parser::parse(tokens) {
                Ok(ast) => {
                    println!("=== AST ===");
                    println!("{:#?}", ast);
                    ast
                }
                Err(error) => {
                    eprintln!("AST error: {}", error);
                    return;
                }
            };
        // backend
            // llvm compiler
    }
}