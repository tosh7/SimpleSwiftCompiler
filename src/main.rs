mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;

use std::fs;
use std::env;
use std::process;
use std::process::Command;

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

    let llvm_ir = codegen::generate_llvm(&ast);
    println!("=== LLVM IR ===");
    println!("{}", llvm_ir);

    let ll_file = "output.ll";
    fs::write(ll_file, &llvm_ir)
        .expect("Failed to write output file");
    
    println!("=== LLVM IR saved to {} ===\n", ll_file);
    
    // Execute with LLVM toolchain (if installed)
    if let Ok(_) = Command::new("lli").output() {
        compile_and_run_llvm(ll_file);
    } else {
        println!("Cannot execute: LLVM toolchain is not installed");
        println!("You can execute with the following commands:");
        println!("  lli {}", ll_file);
        println!("or");
        println!("  llc {} -o output.s", ll_file);
        println!("  clang output.s -o output");
        println!("  ./output");
    }

}

fn compile_and_run_llvm(ll_file: &str) {
    println!("=== LLVM Execution ===");
    
    // Execute directly with lli (LLVM interpreter)
    match Command::new("lli").arg(ll_file).output() {
        Ok(output) => {
            print!("Execution result: ");
            print!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Execution error: {}", e);
        }
    }
    
    // Or compile to native code
    println!("\n=== Native Code Compilation (Optional) ===");
    
    // LLVM IR → Assembly
    if let Ok(_) = Command::new("llc")
        .args(&[ll_file, "-o", "output.s"])
        .output() 
    {
        println!("Generated assembly file: output.s");
        
        // Assembly → Executable
        if let Ok(_) = Command::new("clang")
            .args(&["output.s", "-o", "output"])
            .output()
        {
            println!("Generated executable: output");
            
            // Execute
            if let Ok(output) = Command::new("./output").output() {
                println!("Native execution result: {}", 
                    String::from_utf8_lossy(&output.stdout));
            }
        }
    }
}