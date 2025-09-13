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

    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplicative()?;

        while self.check(TokenType::Plus) || self.check(TokenType::Minus) {
            let operator = if self.check(TokenType::Plus) {
                self.advance();
                BinaryOperator::Add
            } else {
                self.advance()?;
                BinaryOperator::Subtract
            };

            let right = self.parse_multiplicative()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box(right),
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
                self.advance()?;
                BinaryOperator::Devide
            };

            let right = self.parse_primary()?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        if self.check(TokenType::Number) {
            let token = self.advance();
            let value = token.lexeme.parse<i64>()
                .map_err(|_| format!("数値の解析に失敗: {}", token.lexeme))?;
            Ok(Expression::Number(value));
        }

        if self.check(TokenType::LeftParen) {
            self.advance();
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "')' is required")?;
            Ok(expr);
        }

        Err(format!("expression is expected: {:?}", self.peek()))
    }

    pub fn parse(tokens: Vec<Token>) -> Result<AstNode, String> {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
}