use crate::parser_lib::{
    errors::unexpected_token_error::UnexpectedTokenError, lexer::types::token::Token,
};

use super::Parser;

mod parse_description;
mod parse_scope;
mod parse_type;

pub fn parse_type(
    parser: &mut Parser,
    current: &Token,
    next: &Token,
) -> Result<(), UnexpectedTokenError> {
    parse_type::parse_type(parser, current, next)
}

pub fn parse_scope(
    parser: &mut Parser,
    current: &Token,
    next: &Token,
) -> Result<(), UnexpectedTokenError> {
    parse_scope::parse_scope(parser, current, next)
}

pub fn parse_description(
    parser: &mut Parser,
    current: &Token,
    next: &Token,
) -> Result<(), UnexpectedTokenError> {
    parse_description::parse_description(parser, current, next)
}
