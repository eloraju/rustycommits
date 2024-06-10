use crate::parser_lib::parsing::types::Symbol;

#[derive(Debug, Clone)]
pub struct CommitMessage {
    pub topic: Option<Symbol>,
    pub scope: Option<Symbol>,
    pub description: Option<Symbol>,
    pub body: Option<Symbol>,
    pub footers: Option<Vec<Symbol>>,
}

#[derive(Debug, Clone)]
pub enum TextCase {
    Camel,
    Kebab,
    Lower,
    Pascal,
    Scream,
    Snake,
    Upper,
    Unknown,
}
