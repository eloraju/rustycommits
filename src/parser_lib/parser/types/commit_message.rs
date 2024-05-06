use super::Symbol;

#[derive(Debug, Default)]
pub struct CommitMessage {
    commit_type: Option<Symbol>,
    scope: Option<Symbol>,
    description: Option<Symbol>,
    body: Option<Symbol>,
    footer: Option<Symbol>,
}
