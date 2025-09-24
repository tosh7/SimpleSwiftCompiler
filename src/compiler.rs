use crate::lexer;
use crate::token;
use crate::ast;
use crate::parser;
use crate::codegen;
use crate::llvm_backend::LLVMCompiler;

pub struct Compiler {
    backend: LLVMCompiler,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            backend: LLVMCompiler::new(),
        }
    }

    pub fn compile(&self, source: &str) -> Result<(), String> {
        // frontend
        // token
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
                return Err(format!("Lexing error: {}", error));
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
                return Err(format!("AST error: {}", error));
            }
        };
        let llvm_ir = codegen::generate_llvm(&ast);
        println!("=== LLVM IR ===");
        println!("{}", llvm_ir);
        
        // backend
        // llvm compiler
        self.backend.compile_to_executable(&llvm_ir, "output")
            .map_err(|e| e.to_string())
    }
}