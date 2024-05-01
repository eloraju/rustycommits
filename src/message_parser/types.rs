#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(String),
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
}
