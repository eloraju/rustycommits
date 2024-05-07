use itertools::Itertools;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token};

use super::{
    parser_functions::{parse_body, parse_description, parse_scope, parse_type},
    types::CommitMessage,
};

pub fn process(tokens: Vec<Token>) -> Result<CommitMessage, SyntaxError> {
    let mut tokens_iter = tokens.into_iter().multipeek();
    let commit_type = parse_type(&mut tokens_iter)?;
    let scope = parse_scope(&mut tokens_iter)?;
    let description = parse_description(&mut tokens_iter)?;
    let body = parse_body(&mut tokens_iter)?;

    Ok(CommitMessage {
        commit_type: Some(commit_type),
        scope,
        description: Some(description),
        body,
    })
}
