use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Word,
    Bang,
    Colon,
    Hash,
    NewLine,
    ParenthesisClose,
    ParenthesisOpen,
    Space,
    None,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Word => write!(f, "word"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Hash => write!(f, "#"),
            TokenType::NewLine => write!(f, "newline"),
            TokenType::ParenthesisClose => write!(f, ")"),
            TokenType::ParenthesisOpen => write!(f, "("),
            TokenType::Space => write!(f, " "),
            TokenType::None => write!(f, "None"),
        }
    }
}
