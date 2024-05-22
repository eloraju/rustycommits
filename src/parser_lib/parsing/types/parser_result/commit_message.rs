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

#[derive(Debug, Clone)]
pub struct Topic {
    pub text: String,
    pub case: TextCase,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub text: String,
    pub case: TextCase,
    pub start_delimiter: String,
    pub end_delimiter: String,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct Description {
    pub text: String,
    pub case: TextCase,
    pub start_delimiter: String,
    pub braking_change: bool,
    pub length: usize,
}
