use std::vec::IntoIter;

use itertools::{Itertools, MultiPeek};

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

pub fn has_footer_start(tokens: &mut MultiPeek<IntoIter<Token>>) -> bool {
    tokens.reset_peek();
    match tokens.peek() {
        Some(Token::Word(_)) => match tokens.peek() {
            Some(Token::ColonSpace(_)) | Some(Token::SpaceHash(_)) => true,
            _ => false,
        },
        _ => false,
    }
}

fn parse_key(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Option<Token>, SyntaxError> {
    let current = tokens.next();
    let next = tokens.peek();
    match (&current, next) {
        (Some(Token::Word(_)), Some(Token::ColonSpace(_)))
        | (Some(Token::Word(_)), Some(Token::SpaceHash(_))) => Ok(Some(current.unwrap())),
        (Some(_), _) => Err(SyntaxError::expected_string(current.unwrap())),
        (None, _) => Ok(None),
    }
}

fn parse_delimiter(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::ColonSpace(_)) | Some(Token::SpaceHash(_)) => Ok(current.unwrap()),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
        _ => Err(SyntaxError::UnexpectedTokenError(
            current.unwrap(),
            "':<space> ' or '<space>#'".to_string(),
        )),
    }
}

fn take_words(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Vec<Token>, SyntaxError> {
    let mut text_tokens: Vec<Token> = tokens
        .take_while_ref(|token| match token {
            Token::NewLine(_) => false,
            _ => true,
        })
        .collect();

    // This should always be a newline or None
    let newline = tokens.next();
    match &newline {
        Some(Token::NewLine(_)) => match tokens.peek() {
            Some(Token::NewLine(_)) => Err(SyntaxError::expected_string(tokens.next().unwrap())),
            Some(_) => {
                text_tokens.push(newline.unwrap());
                if has_footer_start(tokens) {
                    return Ok(text_tokens);
                }
                text_tokens.extend(take_words(tokens)?);
                return Ok(text_tokens);
            }
            None => {
                text_tokens.push(newline.unwrap());
                Ok(text_tokens)
            }
        },
        Some(token) => Err(SyntaxError::UnexpectedTokenError(
            token.clone(),
            "this not to get here in the first place.".to_string(),
        )),
        None => Ok(text_tokens),
    }
}

// This runs after every new line
fn recurse_footer(
    tokens: &mut MultiPeek<IntoIter<Token>>,
) -> Result<Option<Vec<Symbol>>, SyntaxError> {
    if let Some(key) = parse_key(tokens)? {
        let footer = Symbol::Footer {
            key: key,
            delimiter: parse_delimiter(tokens)?,
            text_tokens: take_words(tokens)?,
        };

        let mut footers: Vec<Symbol> = Vec::new();
        footers.push(footer);

        tokens.reset_peek();
        if has_footer_start(tokens) {
            if let Some(next_footer) = recurse_footer(tokens)? {
                footers.extend(next_footer);
            }
        }

        return Ok(Some(footers));
    }

    return Ok(None);
}

pub fn parse_footer(
    tokens: &mut MultiPeek<IntoIter<Token>>,
) -> Result<Option<Vec<Symbol>>, SyntaxError> {
    return Ok(recurse_footer(tokens)?);
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::test_utils::TestTokens;

    use super::*;

    #[test]
    fn should_parse_single_footer() {
        let mut tokens = TestTokens::new()
            .word("footer")
            .colon_space()
            .word("this")
            .generate_iter();
        let symbol = parse_footer(&mut tokens).unwrap().unwrap().pop().unwrap();
        assert!(matches!(symbol, Symbol::Footer { .. }));
        assert_eq!(symbol.value(), "footer: this");
    }

    #[test]
    fn should_parse_multiple_footers() {
        let mut tokens = TestTokens::new()
            .word("footer")
            .colon_space()
            .word("this")
            .newline()
            .word("another-footer")
            .space_hash()
            .word("12")
            .generate_iter();
        let symbols = parse_footer(&mut tokens).unwrap().unwrap();
        assert_eq!(symbols.len(), 2);
        assert!(matches!(symbols[0], Symbol::Footer { .. }));
        assert!(matches!(symbols[1], Symbol::Footer { .. }));
        assert_eq!(symbols[0].value(), "footer: this\n");
        assert_eq!(symbols[1].value(), "another-footer #12");
    }
}
