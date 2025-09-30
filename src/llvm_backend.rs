use std::fs;
use std::process::Command;
use crate::options::CompilerOption;

pub struct LLVMCompiler {
    _output_dir: String,
    options: Vec<CompilerOption>,
}

impl LLVMCompiler {
  pub fn new(options:Vec<CompilerOption>) -> Self {
    LLVMCompiler {
      _output_dir: "target/llvm".to_string(),
      options: options,
    }
  }

  pub fn compile_to_executable(&self, llvm_ir: &str, _output: &str) -> Result<(), &str> {
    // Create output directory
    let output_dir = "target/llvm";
    fs::create_dir_all(output_dir)
        .expect("Failed to create output directory");
    
    let ll_file = format!("{}/output.ll", output_dir);
    let s_file = format!("{}/output.s", output_dir);
    let exec_file = format!("{}/output", output_dir);
    
    fs::write(&ll_file, &llvm_ir)
        .expect("Failed to write output file");
    
    if self.options.contains(&CompilerOption::Verbose) {
        println!("=== LLVM IR saved to {} ===\n", ll_file);
    }
    
    // Execute with LLVM toolchain (if installed)
    if let Ok(_) = Command::new("lli").output() {
        Self::compile_and_run_llvm(self, &ll_file, &s_file, &exec_file);
        Ok(())
    } else {
        eprintln!("Cannot execute: LLVM toolchain is not installed");
        eprintln!("You can execute with the following commands:");
        eprintln!("  lli {}", ll_file);
        eprintln!("or");
        eprintln!("  llc {} -o {}", ll_file, s_file);
        eprintln!("  clang {} -o {}", s_file, exec_file);
        eprintln!("  {}", exec_file);
        Err("LLVM toolchain not installed. Please install LLVM to execute the generated code.")
    }
  }

  fn compile_and_run_llvm(&self, ll_file: &str, s_file: &str, exec_file: &str) {
    println!("=== LLVM Execution ===");
    
    // Execute directly with lli (LLVM interpreter)
    match Command::new("lli").arg(ll_file).output() {
        Ok(output) => {
            if self.options.contains(&CompilerOption::Verbose) {
                print!("Execution result: ");
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Execution error: {}", e);
        }
    }
    
    // Or compile to native code
    if self.options.contains(&CompilerOption::Verbose) {
        println!("\n=== Native Code Compilation (Optional) ===");
    }
    
    // LLVM IR → Assembly
    if let Ok(_) = Command::new("llc")
        .args(&[ll_file, "-o", s_file])
        .output() 
    {
        if self.options.contains(&CompilerOption::Verbose) {
            println!("Generated assembly file: {}", s_file);
        }
        
        // Assembly → Executable
        if let Ok(_) = Command::new("clang")
            .args(&[s_file, "-o", exec_file])
            .output()
        {
            if self.options.contains(&CompilerOption::Verbose) {
                println!("Generated executable: {}", exec_file);
            }
            
            // Execute
            if let Ok(output) = Command::new(exec_file).output() {
                println!("Native execution result: {}", 
                    String::from_utf8_lossy(&output.stdout));
            }
        }
    }
  }
}