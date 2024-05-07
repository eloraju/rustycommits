use std::vec::IntoIter;

use itertools::{Itertools, MultiPeek};

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

fn take_words(
    tokens: &mut MultiPeek<IntoIter<Token>>,
    buffer: Option<Vec<Token>>,
) -> Result<Vec<Token>, SyntaxError> {
    let mut text_tokens = buffer.unwrap_or_default();
    text_tokens.extend(
        tokens
            .take_while_ref(|token| {
                dbg!(token);
                match token {
                    Token::NewLine(_) => false,
                    _ => true,
                }
            })
            .collect_vec(),
    );

    match tokens.peek() {
        Some(Token::NewLine(_)) => match tokens.peek() {
            Some(Token::NewLine(_)) => Ok(text_tokens),
            Some(_) => take_words(tokens, Some(text_tokens)),
            None => Ok(text_tokens),
        },
        Some(_) => take_words(tokens, Some(text_tokens)),
        None => Ok(text_tokens),
    }
}

fn check_end_delimeter(
    tokens: &mut MultiPeek<IntoIter<Token>>,
) -> Result<Option<Vec<Token>>, SyntaxError> {
    let mut end_delimeter: Vec<Token> = Vec::new();
    let current = tokens.next();
    let next = tokens.peek();
    match (&current, next) {
        (Some(Token::NewLine(_)), Some(Token::NewLine(_))) => {
            end_delimeter.push(current.unwrap());
            end_delimeter.push(tokens.next().unwrap());
            Ok(Some(end_delimeter))
        }
        (Some(Token::NewLine(_)), None) => {
            end_delimeter.push(current.unwrap());
            Ok(Some(end_delimeter))
        }
        (None, _) => Ok(None),
        (Some(token), _) => Err(SyntaxError::UnexpectedTokenError(token.clone())),
    }
}

pub fn parse_body(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Option<Symbol>, SyntaxError> {
    let text_tokens = take_words(tokens, None)?;
    let end_delimeter = check_end_delimeter(tokens)?;

    Ok(Some(Symbol::Body {
        text_tokens,
        end_delimeter,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::TestTokens;

    #[test]
    fn should_parse_body() {
        let mut tokens = TestTokens::new()
            .word("this")
            .space()
            .word("is")
            .space()
            .word("a")
            .space()
            .word("body")
            .newline()
            .newline()
            .generate_iter();

        let result = parse_body(&mut tokens);
        let symbol = result.unwrap().unwrap();
        match &symbol {
            Symbol::Body {
                text_tokens,
                end_delimeter: end_separator,
            } => {
                assert_eq!(text_tokens.len(), 7);
                assert_eq!(end_separator.as_ref().unwrap().len(), 2);
                assert_eq!(symbol.raw_value(), "this is a body\n\n");
                assert_eq!(symbol.value(), "this is a body");
            }
            _ => {
                panic!("Invalid symbol type");
            }
        }
    }
}
