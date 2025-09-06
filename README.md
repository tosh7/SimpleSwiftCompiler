# Simple Swift Compiler

A simple Swift compiler implementation written in Rust. This project implements a basic lexer, parser, and code generator for a subset of the Swift programming language.

## Features

- **Lexer**: Tokenizes Swift source code
- **Parser**: Builds an Abstract Syntax Tree (AST)
- **Code Generator**: Generates target code from the AST
- **Semantic Analysis**: Type checking and semantic validation

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
3. Display the source code and tokens

### Example Output

```
Source code:
print(42)
Tokens:
0: Token { token_type: Print, lexeme: "print" }
1: Token { token_type: LeftParen, lexeme: "(" }
2: Token { token_type: Number, lexeme: "42" }
3: Token { token_type: RightParen, lexeme: ")" }
4: Token { token_type: EOF, lexeme: "" }
```

## Supported Features

Currently, the compiler supports:
- `print` statements
- Integer literals
- Basic tokenization

## Requirements

- Rust 1.56 or later
- Cargo

## Development

To check for compilation errors without building:

```bash
cargo check
```

## License

Apache 2.0