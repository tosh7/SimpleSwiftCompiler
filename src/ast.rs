#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Proguram(Vec<Statement>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(i64),
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
    Devide,
}