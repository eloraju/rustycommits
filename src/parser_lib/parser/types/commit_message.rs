use super::Symbol;

#[derive(Debug, Default, Clone)]
pub struct CommitMessage {
    pub topic: Option<Symbol>,
    pub scope: Option<Symbol>,
    pub description: Option<Symbol>,
    pub body: Option<Symbol>,
    pub footers: Option<Vec<Symbol>>,
}

#[derive(Debug)]
pub struct CommitMessageStr {
    pub topic: Option<String>,
    pub scope: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub footers: Option<Vec<String>>,
}

impl CommitMessage {
    pub fn to_no_delim_strings(msg: CommitMessage) -> CommitMessageStr {
        CommitMessageStr {
            topic: msg.topic.map(|x| x.no_delims_string()),
            scope: msg.scope.map(|x| x.no_delims_string()),
            description: msg.description.map(|x| x.no_delims_string()),
            body: msg.body.map(|x| x.no_delims_string()),
            footers: msg
                .footers
                .map(|x| x.into_iter().map(|x| x.no_delims_string()).collect()),
        }
    }

    pub fn to_full_strings(msg: CommitMessage) -> CommitMessageStr {
        CommitMessageStr {
            topic: msg.topic.map(|x| x.full_string()),
            scope: msg.scope.map(|x| x.full_string()),
            description: msg.description.map(|x| x.full_string()),
            body: msg.body.map(|x| x.full_string()),
            footers: msg
                .footers
                .map(|x| x.into_iter().map(|x| x.full_string()).collect()),
        }
    }
}
