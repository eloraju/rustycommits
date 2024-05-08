use std::vec::IntoIter;

use itertools::{Itertools, MultiPeek};

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

fn check_bang(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Option<Token>, SyntaxError> {
    let current = tokens.peek();
    match current {
        Some(Token::Bang(_)) => Ok(tokens.next()),
        Some(_) => Ok(None),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

fn check_start_delimiter(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::ColonSpace(_)) => Ok(current.unwrap()),
        Some(token) => match token {
            Token::Colon(_) => Err(SyntaxError::expected_space(token)),
            _ => Err(SyntaxError::expected_colon(token)),
        },
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

fn take_words(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Vec<Token>, SyntaxError> {
    let next = tokens.peek();
    match next {
        Some(Token::NewLine(_)) => {
            return Err(SyntaxError::UnexpectedTokenError(
                tokens.next().unwrap(),
                "anything but this".to_string(),
            ))
        }
        Some(_) => {}
        None => return Err(SyntaxError::UnexpectedEndOfFileError),
    };

    Ok(tokens
        .take_while_ref(|token| match token {
            Token::NewLine(_) => false,
            _ => true,
        })
        .collect_vec())
}

fn check_end_delimiter(
    tokens: &mut MultiPeek<IntoIter<Token>>,
) -> Result<Option<Vec<Token>>, SyntaxError> {
    let mut end_delimiter: Vec<Token> = Vec::new();
    let current = tokens.next();
    let next = tokens.peek();
    match (&current, next) {
        (Some(Token::NewLine(_)), Some(Token::NewLine(_))) => {
            end_delimiter.push(current.unwrap());
            end_delimiter.push(tokens.next().unwrap());
            Ok(Some(end_delimiter))
        }
        (Some(Token::NewLine(_)), None) => {
            end_delimiter.push(current.unwrap());
            Ok(Some(end_delimiter))
        }
        (None, _) => Ok(None),
        (Some(Token::NewLine(_)), Some(_)) => {
            Err(SyntaxError::expected_newline(tokens.next().unwrap()))
        }
        (Some(_), _) => Err(SyntaxError::expected_newline(current.unwrap())),
    }
}

pub fn parse_description(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Symbol, SyntaxError> {
    let braking_change_token = check_bang(tokens)?;
    let start_delimeter = check_start_delimiter(tokens)?;
    let text_tokens = take_words(tokens)?;
    let end_delimiter = check_end_delimiter(tokens)?;

    Ok(Symbol::Description {
        start_delimeter,
        text_tokens,
        end_delimiter,
        braking_change_token,
    })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::parser_lib::{parser::types::Symbol, test_utils::TestTokens};

    use super::parse_description;
    #[test]
    fn should_parse_description() {
        let mut tokens = TestTokens::new()
            .colon()
            .space()
            .word("description")
            .space()
            .word("is")
            .space()
            .word("this")
            .newline()
            .newline()
            .generate();
        dbg!(&tokens);
        let res = parse_description(&mut tokens.into_iter().multipeek());
        let symbol = res.unwrap();
        match &symbol {
            Symbol::Description {
                start_delimeter,
                text_tokens,
                end_delimiter,
                braking_change_token: _,
            } => {
                assert_eq!(start_delimeter.len(), 2);
                assert_eq!(text_tokens.len(), 5);
                assert_eq!(end_delimiter.as_ref().unwrap().len(), 2);
                assert_eq!(symbol.raw_value(), ": description is this\n\n");
                assert_eq!(symbol.value().to_string(), "description is this");
            }
            _ => panic!("Error: {:?}", symbol),
        }
    }
}
