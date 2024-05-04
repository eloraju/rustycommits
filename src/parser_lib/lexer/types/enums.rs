use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Word,
    Bang,
    Colon,
    Dash,
    Hash,
    NewLine,
    ParenthesisClose,
    ParenthesisOpen,
    Space,
    Underscore,
    EOF,
    None,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Word => write!(f, "word"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Dash => write!(f, "-"),
            TokenType::Hash => write!(f, "#"),
            TokenType::NewLine => write!(f, "newline"),
            TokenType::ParenthesisClose => write!(f, ")"),
            TokenType::ParenthesisOpen => write!(f, ")"),
            TokenType::Space => write!(f, "space"),
            TokenType::Underscore => write!(f, "_"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::None => write!(f, "None"),
        }
    }
}
