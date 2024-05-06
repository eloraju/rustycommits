use std::fmt::Display;

use super::enums::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub value: Box<str>,
    pub token_type: TokenType,
    pub start_i: usize,
    pub end_i: usize,
    pub len: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        value: Box<str>,
        start_i: usize,
        end_i: usize,
        len: usize,
    ) -> Token {
        Token {
            token_type,
            value,
            start_i,
            end_i,
            len,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: TokenType::None,
            start_i: Default::default(),
            end_i: Default::default(),
            len: Default::default(),
            value: Default::default(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::Word => write!(f, "{}", self.value),
            TokenType::Bang => write!(f, "!"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Hash => write!(f, "#"),
            TokenType::NewLine => write!(f, "\n"),
            TokenType::ParenthesisClose => write!(f, ")"),
            TokenType::ParenthesisOpen => write!(f, "("),
            TokenType::Space => write!(f, " "),
            TokenType::None => write!(f, "NONE"),
        }
    }
}