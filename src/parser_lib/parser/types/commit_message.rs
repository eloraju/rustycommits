use super::Symbol;

#[derive(Debug, Default)]
pub struct CommitMessage {
    pub commit_type: Option<Symbol>,
    pub scope: Option<Symbol>,
    pub description: Option<Symbol>,
    pub body: Option<Symbol>,
    //pub footer: Option<Symbol>,
}
