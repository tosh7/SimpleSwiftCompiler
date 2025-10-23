# Simple Swift Compiler

A simple Swift compiler implementation written in Rust. This project implements a basic lexer, parser, and LLVM code generator for a subset of the Swift programming language.

## Features

- **Lexer**: Tokenizes Swift source code
- **Parser**: Builds an Abstract Syntax Tree (AST) with operator precedence
- **LLVM Code Generator**: Generates LLVM IR from the AST
- **Native Execution**: Compiles and executes code via LLVM toolchain
- **Arithmetic Operations**: Supports basic math operations (+, -, *, /)
- **Variable Declaration**: Support for `let` declarations with type inference
- **Variable References**: Use declared variables in expressions
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
│   ├── ArithmeticOperators.swift  # Arithmetic operations example
│   └── VariablesAssigment.swift   # Variable declaration example
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


## Supported Features

Currently, the compiler supports:
- `print` statements
- Integer literals
- Variable declarations with `let`
- Variable references in expressions
- Arithmetic expressions (+, -, *, /)
- Expression parsing with operator precedence
- Abstract Syntax Tree (AST) generation
- LLVM IR generation with stack allocation for variables

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