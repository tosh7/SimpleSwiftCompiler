
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program(Vec<Statement>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print(Expression),
    VarDecl {
        name: String,
        value: Expression,
        is_mutable: bool,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    Expression(Expression)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(i64),
    Variable(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}