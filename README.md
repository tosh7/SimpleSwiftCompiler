# Simple Swift Compiler

A simple Swift compiler implementation written in Rust. This project implements a basic lexer, parser, and LLVM code generator for a subset of the Swift programming language.

## Features

- **Lexer**: Tokenizes Swift source code
- **Parser**: Builds an Abstract Syntax Tree (AST) with operator precedence
- **LLVM Code Generator**: Generates LLVM IR from the AST
- **Native Execution**: Compiles and executes code via LLVM toolchain
- **Arithmetic Operations**: Supports basic math operations (+, -, *, /)

## Project Structure

```
simple-swift-compiler/
├── src/
│   ├── main.rs      # Entry point
│   ├── lexer.rs     # Lexical analyzer
│   ├── token.rs     # Token definitions
│   ├── parser.rs    # Parser implementation
│   ├── ast.rs       # AST node definitions
│   ├── semantic.rs  # Semantic analyzer
│   └── codegen.rs   # Code generation
├── example/
│   └── Test.swift   # Example Swift code
└── Cargo.toml       # Project configuration
```

## Building the Project

```bash
cargo build
```

## Running the Compiler

```bash
cargo run example/Test.swift
```

This will:
1. Read the Swift source file
2. Tokenize the input  
3. Parse the tokens into an AST
4. Generate LLVM IR code
5. Execute the code using LLVM toolchain
6. Optionally compile to native executable

### Example Output

```
Source code:
print(42+1)
Tokens:
0: Token { token_type: Print, lexeme: "print" }
1: Token { token_type: LeftParen, lexeme: "(" }
2: Token { token_type: Number, lexeme: "42" }
3: Token { token_type: Plus, lexeme: "+" }
4: Token { token_type: Number, lexeme: "1" }
5: Token { token_type: RightParen, lexeme: ")" }
6: Token { token_type: EOF, lexeme: "" }
=== AST ===
Program(
    [
        Print(
            Binary {
                left: Number(
                    42,
                ),
                operator: Add,
                right: Number(
                    1,
                ),
            },
        ),
    ],
)
=== LLVM IR ===
; ModuleID = 'swift_module'
source_filename = "swift_source"

declare i32 @printf(i8*, ...)

@.str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %1 = add i32 42, 1
  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %1)
  ret i32 0
}

=== LLVM IR saved to output.ll ===

=== LLVM Execution ===
Execution result: 43

=== Native Code Compilation (Optional) ===
Generated assembly file: output.s
Generated executable: output
Native execution result: 43
```

## Supported Features

Currently, the compiler supports:
- `print` statements
- Integer literals
- Arithmetic expressions (+, -, *, /)
- Expression parsing with operator precedence
- Abstract Syntax Tree (AST) generation

## Requirements

- Rust 1.56 or later (Edition 2024)
- Cargo
- LLVM toolchain (for code execution)

### Installing LLVM on macOS

```bash
brew install llvm
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
```

### Installing LLVM on Linux

```bash
sudo apt-get install llvm  # Ubuntu/Debian
# or
sudo yum install llvm      # RHEL/CentOS
```

## Development

To check for compilation errors without building:

```bash
cargo check
```

## Generated Files

When you run the compiler, it generates:
- `output.ll` - LLVM IR code
- `output.s` - Assembly code (optional)
- `output` - Native executable (optional)

## License

Apache 2.0