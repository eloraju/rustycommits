use super::Symbol;

#[derive(Debug, Default)]
pub struct CommitMessage {
    commit_type: Option<Symbol>,
    scope: Option<Symbol>,
    description: Option<Symbol>,
    body: Option<Symbol>,
    footer: Option<Symbol>,
}

impl From<Vec<Symbol>> for CommitMessage {
    fn from(symbols: Vec<Symbol>) -> Self {
        let mut commit_message = CommitMessage::default();
        for symbol in symbols {
            match symbol {
                Symbol::Type { .. } => commit_message.commit_type = Some(symbol),
                Symbol::Scope { .. } => commit_message.scope = Some(symbol),
                Symbol::Description { .. } => commit_message.description = Some(symbol),
                Symbol::Body { .. } => commit_message.body = Some(symbol),
                Symbol::Footer { .. } => commit_message.footer = Some(symbol),
            }
        }
        commit_message
    }
}
