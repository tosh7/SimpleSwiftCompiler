# Simple Swift Compiler

A simple Swift compiler implementation written in Rust. This project implements a basic lexer, parser, and LLVM code generator for a subset of the Swift programming language.

## Features

- **Lexer**: Tokenizes Swift source code
- **Parser**: Builds an Abstract Syntax Tree (AST) with operator precedence
- **LLVM Code Generator**: Generates LLVM IR from the AST
- **Native Execution**: Compiles and executes code via LLVM toolchain
- **Arithmetic Operations**: Supports basic math operations (+, -, *, /)
- **Command-line Options**: Verbose mode for detailed compilation output

## Project Structure

```
simple-swift-compiler/
├── src/
│   ├── main.rs          # Entry point and CLI argument parsing
│   ├── compiler.rs      # Main compiler orchestrator
│   ├── lexer.rs         # Lexical analyzer
│   ├── token.rs         # Token definitions
│   ├── parser.rs        # Parser implementation
│   ├── ast.rs           # AST node definitions
│   ├── codegen.rs       # LLVM IR code generation
│   ├── llvm_backend.rs  # LLVM backend implementation
│   └── options.rs       # Compiler options and flags
├── example/
│   └── Test.swift       # Example Swift code
├── target/llvm/         # Generated LLVM IR files
│   ├── output.ll        # LLVM IR output
│   ├── output.s         # Assembly output (optional)
│   └── output           # Executable (optional)
└── Cargo.toml           # Project configuration
```

## Building the Project

```bash
cargo build
```

## Running the Compiler

### Basic Usage

```bash
cargo run -- example/Test.swift
```

### Verbose Mode

For detailed compilation output including tokens, AST, and LLVM IR:

```bash
cargo run -- --verbose example/Test.swift
# or
cargo run -- -v example/Test.swift
```

### Command-line Options

- `--verbose` or `-v`: Enable verbose output showing all compilation stages

This will:
1. Read the Swift source file
2. Tokenize the input (Frontend: Lexical Analysis)
3. Parse the tokens into an AST (Frontend: Syntax Analysis)
4. Generate LLVM IR code (Frontend: IR Generation)
5. Save LLVM IR to `target/llvm/output.ll`
6. Execute the code using LLVM toolchain (if installed)

### Example Output

For the input file `example/Test.swift`:
```swift
print(42)
print(42+2)
print(42-2)
print(42*2)
print(42/2)
```

The compiler outputs:
```
Source code:
print(42)
print(42+2)
print(42-2)
print(42*2)
print(42/2)

Tokens:
0: Token { token_type: Print, lexeme: "print" }
1: Token { token_type: LeftParen, lexeme: "(" }
2: Token { token_type: Number, lexeme: "42" }
...

=== AST ===
Program([
    Print(Number(42)),
    Print(Binary { left: Number(42), operator: Add, right: Number(2) }),
    Print(Binary { left: Number(42), operator: Subtract, right: Number(2) }),
    Print(Binary { left: Number(42), operator: Multiply, right: Number(2) }),
    Print(Binary { left: Number(42), operator: Divide, right: Number(2) }),
])

=== LLVM IR ===
; ModuleID = 'swift_module'
source_filename = "swift_source"

declare i32 @printf(i8*, ...)

@.str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 42)
  %2 = add i32 42, 2
  %3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %2)
  %4 = sub i32 42, 2
  %5 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %4)
  %6 = mul i32 42, 2
  %7 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %6)
  %8 = sdiv i32 42, 2
  %9 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %8)
  ret i32 0
}

=== LLVM IR saved to target/llvm/output.ll ===

Execution result (with LLVM):
42
44
40
84
21
```

## Supported Features

Currently, the compiler supports:
- `print` statements
- Integer literals
- Arithmetic expressions (+, -, *, /)
- Expression parsing with operator precedence
- Abstract Syntax Tree (AST) generation

## Requirements

- Rust 1.56 or later
- Cargo
- LLVM toolchain (optional, for code execution)

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

## Architecture

The compiler follows a modular architecture:

1. **Frontend (Source → LLVM IR)**:
   - `lexer.rs`: Converts source code into tokens
   - `parser.rs`: Builds AST from tokens with operator precedence
   - `codegen.rs`: Generates LLVM IR from AST

2. **Backend (LLVM IR → Execution)**:
   - `llvm_backend.rs`: Handles LLVM toolchain interaction
   - Saves IR to files
   - Executes via LLVM interpreter (lli)
   - Can compile to native code (llc + clang)

3. **Orchestration**:
   - `compiler.rs`: Coordinates the compilation pipeline
   - `main.rs`: CLI interface

## Generated Files

When you run the compiler, it generates:
- `target/llvm/output.ll` - LLVM IR code
- `target/llvm/output.s` - Assembly code (when using llc)
- `target/llvm/output` - Native executable (when using clang)

## License

Apache 2.0