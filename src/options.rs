#[derive(Debug, Clone, PartialEq)]
pub enum CompilerOption {
    Verbose,
}

impl CompilerOption {
    pub fn from_literal(option_literal: &str) -> Option<Self> {
        match option_literal {
            "-v" | "--verbose" => Some(CompilerOption::Verbose),
            _ => None
        }
    }
}