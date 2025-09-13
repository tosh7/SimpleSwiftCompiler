#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Print,
    Number,
    LeftParen, // (
    RightParen, // )
    EOF,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Token { token_type, lexeme}
    }
}