use crate::token::{Token, TokenType};
use crate::ast::{AstNode, Statement, Expression, BinaryOperator};

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
            self.peak().token_type == token_type
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

        Ok(AstNode::Program(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peak().token_type {
            TokenType::Print => {
                self.parse_print()
            }
            TokenType::Let => {
                self.parse_declaration()
            }
            TokenType::Var => {
                self.parse_declaration()
            }
            _ => {
                Err("Unexpected token in statement".to_string())
            }
        }
    }

    fn parse_declaration(&mut self) -> Result<Statement, String> {
        // Check if it's 'let' or 'var'
        let is_mutable = if self.check(TokenType::Let) {
            self.advance();
            false
        } else if self.check(TokenType::Var) {
            self.advance();
            true
        } else {
            return Err("Expected 'let' or 'var' keyword".to_string());
        };

        // Get variable name
        let name_token = self.consume(TokenType::Identifier, "Expected variable name")?;
        let name = name_token.lexeme;

        // Skip optional type annotation for now (: Type)
        if self.check(TokenType::Colon) {
            self.advance(); // consume ':'
            // For now, skip the type identifier
            if self.check(TokenType::Identifier) {
                self.advance(); // consume type name
            }
        }
        // Expect assignment
        self.consume(TokenType::Assign, "Expected '=' in variable declaration")?;

        // Parse the value expression
        let value = self.parse_expression()?;

        Ok(Statement::VarDecl {
            name,
            value,
            is_mutable,
        })
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::Print, "Expected 'print' keyword")?;
        self.consume(TokenType::LeftParen, "Expected '(' after 'print'")?;

        let expr = self.parse_expression()?;

        self.consume(TokenType::RightParen, "Expected ')'")?;
        
        Ok(Statement::Print(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplicative()?;

        while self.check(TokenType::Plus) || self.check(TokenType::Minus) {
            let operator = if self.check(TokenType::Plus) {
                self.advance();
                BinaryOperator::Add
            } else {
                self.advance();
                BinaryOperator::Subtract
            };

            let right = self.parse_multiplicative()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;

        while self.check(TokenType::Star) || self.check(TokenType::Slash) {
            let operator = if self.check(TokenType::Star) {
                self.advance();
                BinaryOperator::Multiply
            } else {
                self.advance();
                BinaryOperator::Divide
            };

            let right = self.parse_primary()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        if self.check(TokenType::Number) {
            let token = self.advance();
            let value = token.lexeme.parse::<i64>()
                .map_err(|_| format!("Failed to parse number: {}", token.lexeme))?;
            return Ok(Expression::Number(value));
        }

        if self.check(TokenType::Identifier) {
            let token = self.advance();
            return Ok(Expression::Variable(token.lexeme));
        }

        if self.check(TokenType::LeftParen) {
            self.advance();
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "')' is required")?;
            return Ok(expr);
        }

        Err(format!("expression is expected: {:?}", self.peak()))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<AstNode, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}