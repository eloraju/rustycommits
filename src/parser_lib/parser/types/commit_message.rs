use super::Symbol;

#[derive(Debug)]
pub struct CommitMessage {
    topic: Option<Symbol>,
    scope: Option<Symbol>,
    description: Option<Symbol>,
    body: Option<Symbol>,
    footer: Option<Symbol>,
}
