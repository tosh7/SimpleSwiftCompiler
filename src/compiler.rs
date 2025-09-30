use crate::codegen;
use crate::lexer;
use crate::parser;
use crate::llvm_backend::LLVMCompiler;
use crate::options::CompilerOption;

pub struct Compiler {
    backend: LLVMCompiler,
    options: Vec<CompilerOption>,
}

impl Compiler {
    pub fn new(options: Vec<CompilerOption>) -> Self {
        Compiler {
            backend: LLVMCompiler::new(options.clone()),
            options: options,
        }
    }

    pub fn compile(&self, source: &str) -> Result<(), String> {
        // frontend
        // token
        let tokens = match lexer::tokenize(&source) {
            Ok(tokens) => {
                if self.options.contains(&CompilerOption::Verbose) {
                    println!("Tokens:");
                    for(i, token) in tokens.iter().enumerate() {
                        println!("{}: {:?}", i, token);
                    }
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
                if self.options.contains(&CompilerOption::Verbose) {
                    println!("=== AST ===");
                    println!("{:#?}", ast);
                }
                ast
            }
            Err(error) => {
                eprintln!("AST error: {}", error);
                return Err(format!("AST error: {}", error));
            }
        };
        let llvm_ir = codegen::generate_llvm(&ast);
        if self.options.contains(&CompilerOption::Verbose) {
            println!("=== LLVM IR ===");
            println!("{}", llvm_ir);
        }
        
        // backend
        // llvm compiler
        self.backend.compile_to_executable(&llvm_ir, "output")
            .map_err(|e| e.to_string())
    }
}