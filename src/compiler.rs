mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;

use std::fs;
use std::env;
use std::process;
use std::process::Command;

pub struct Compiler {

}

impl Compiler {
    pub fn new(input: Strimg) -> Self {
        Compile {
            
        }
    }

    pub fn compile(&self, source: &str) -> Result<(), String> {
        // frontend
            // token
            // ast

        // backend
            // llvm compiler
    }
}