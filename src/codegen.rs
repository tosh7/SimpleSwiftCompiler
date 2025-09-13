use crate::ast::{AstNode, Statement, Expression, BinaryOperator};

pub struct LLVMCodeGenerator {
    output: String,
    indent_label: usize,
    next_register: i32,
}

impl LLVMCodeGenerator {
    pub fn new() -> Self {
        LLVMCodeGenerator {
            output: String::new(),
            indent_label: 0,
            next_register: 1,
        }
    }

    fn alloc_register(&mut self) -> String {
        let reg = format!("%{}", self.next_register);
        self.next_register += 1;
        reg
    }

    fn emit_indent(&mut self) {
        for _ in 0..self.indent_label {
            self.output.push_str("  ");
        }
    }

    fn emit_line(&mut self, code: &str) {
        self.emit_indent();
        self.output.push_str(code);
        self.output.push('\n');
    }

    fn emit(&mut self, code: &str) {
        self.output.push_str(code);
    }

    fn generate(&mut self, ast: &AstNode) -> String {
        self.output.clear();
        self.next_register = 1;

        // LLVM IR header
        self.emit_line("; ModuleID = 'swift_module'");
        self.emit_line("source_filename = \"swift_source\"");
        self.emit_line("");

        // Declare external printf function
        self.emit_line("declare i32 @printf(i8*, ...)");
        self.emit_line("");

        // Format string for printf(global costant)
        self.emit_line("@.str = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\", align 1");
        self.emit_line("");

        // Main function
        self.emit_line("define i32 @main() {");
        self.emit_line("entry:");
        self.indent_label = 1;

        self.visit_node(ast);

        // Return from main
        self.emit_line("ret i32 0");
        self.indent_label = 0;
        self.emit_line("}");

        self.output.clone()
    }

    fn visit_node(&mut self, node: &AstNode) {
        match node {
            AstNode::Program
        }
    }
}