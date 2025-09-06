use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    // MARK - constructors
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current = if chars.is_empty() {
            None
        } else {
            Some(chars[0])
        };

        Lexer {
            input: chars,
            position: 0,
            current_char: current,
        }
    }

    // advance to next character
    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_numeric() {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        number
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.current_char {
            None => Ok(Token::new(TokenType::EOF, String::new())),

            Some(ch) => match ch {
                '(' => {
                    self.advance();
                    Ok(Token::new(TokenType::LeftParen, "(".to_string()))
                }
                ')' => {
                    self.advance();
                    Ok(Token::new(TokenType::RightParen, ")".to_string()))
                }
                '0'..='9' => {
                    let number = self.read_number();
                    Ok(Token::new(TokenType::Number, number))
                }
                'a'..='z' | 'A' ..= 'Z' => {
                    let identifier = self.read_identifier();

                    if identifier == "print" {
                        Ok(Token::new(TokenType::Print, identifier))
                    } else {
                        Err(format!("unknown identifier: {}", identifier))
                    }
                }
                _ => Err(format!("unexpected character: {}", ch)),
            } 
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input.to_string());
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token()?;
        let is_eof = token.token_type == TokenType::EOF;
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    Ok(tokens)
}