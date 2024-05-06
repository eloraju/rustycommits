use itertools::Itertools;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token};

use super::parser_functions::parse_type;

pub struct Parser {}

impl Parser {
    pub fn process(&mut self, tokens: Vec<Token>) -> Result<(), SyntaxError> {
        let mut tokens_iter = tokens.into_iter().multipeek();
        let commit_type = parse_type(&mut tokens_iter);

        return Ok(());
    }
}
