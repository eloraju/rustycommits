#[derive(Debug)]
pub struct CommitMsg {
    pub msg_type: Option<String>,
    pub msg_scope: Option<String>,
    pub msg_description: Option<String>,
    pub msg_body: Option<String>,
    pub msg_footer: Option<String>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Type(String),
    Scope(String),
    Description(String),
    Paragraph(String),
    Word(String),
    NewLine,
    Dash,
    Hash,
    Colon,
    Space,
    OpenParenthesis,
    CloseParenthesis,
}
