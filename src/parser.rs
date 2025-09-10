use create::token::{Token, TokenType};
use create::ast::{AstNode, Statement, Expression, BinaryOperator};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    /// get current token
    fn peak(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// check if current token is specified type
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    /// move to next token
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.tokens[self.current - 1].clone()
    }

    /// check if moved to EOF
    fn is_at_end(&self) -> bool {
        self.peak().token_type == TokenType::EOF
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} at token {:?}", message, self.peak()))
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(AstNode::Proguram(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Print, "Expected 'print' keyword")?;
        self.consume(TokenType::LeftParen, "Expected '(' after 'print'")?;

        let expr = self.parse_expression()?;

        self.consume(TokenType::RightParen, "Expected ')'")?;
        
        Ok(Statement::Print(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_additive()
    }
}