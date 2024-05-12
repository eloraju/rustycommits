use crate::parser_lib::parser::types::Symbol;

#[derive(Debug, Clone)]
pub struct CommitMessage {
    pub topic: Option<Symbol>,
    pub scope: Option<Symbol>,
    pub description: Option<Symbol>,
    pub body: Option<Symbol>,
    pub footers: Option<Vec<Symbol>>,
}
