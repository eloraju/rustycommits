use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

pub fn is_footer_start(tokens: &mut MultiPeek<IntoIter<Token>>) -> bool {
    match tokens.peek() {
        Some(Token::Word(_)) => match tokens.peek() {
            Some(Token::ColonSpace(_)) | Some(Token::SpaceHash(_)) => true,
            _ => false,
        },
        _ => false,
    }
}

fn parse_key(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Option<Token>, SyntaxError> {
    if !is_footer_start(tokens) {
        return Ok(None);
    }

    let current = tokens.next();
    let next = tokens.peek();
    match (&current, next) {
        (Some(Token::Word(_)), Some(Token::ColonSpace(_))) => return Ok(Some(current.unwrap())),
        (Some(Token::Word(_)), Some(Token::Space(_))) => match tokens.peek() {
            Some(Token::Hash(_)) => return Ok(Some(current.unwrap())),
            _ => Err(SyntaxError::expected_colon(current.unwrap())),
        },
        (Some(_), _) => Err(SyntaxError::expected_string(current.unwrap())),
        (None, _) => Ok(None),
    }
}

fn parse_delimiter(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::ColonSpace(_)) | Some(Token::SpaceHash(_)) => Ok(current.unwrap()),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
        _ => Err(SyntaxError::expected_colon(current.unwrap())),
    }
}

fn parse_value(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::Word(_)) => Ok(current.unwrap()),
        Some(token) => Err(SyntaxError::expected_string(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

pub fn parse_tags(
    tokens: &mut MultiPeek<IntoIter<Token>>,
) -> Result<Option<Vec<Symbol>>, SyntaxError> {
    let mut tags: Vec<Symbol> = Vec::new();
    while let Some(key) = parse_key(tokens)? {
        let delimiter = parse_delimiter(tokens)?;
        let value = parse_value(tokens)?;
        tags.push(Symbol::Footer {
            key,
            delimiter,
            value,
        });
    }

    if tags.len() == 0 {
        return Ok(None);
    }

    return Ok(Some(tags));
}
