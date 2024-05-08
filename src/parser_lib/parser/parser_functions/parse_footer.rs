use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

pub fn parse_tags(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Vec<Symbol>, SyntaxError> {
    return Ok(Vec::new());
}
