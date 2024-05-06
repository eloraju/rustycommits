use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

pub fn parse_body(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Option<Symbol>, SyntaxError> {
    let mut text_tokens: Vec<Token> = Vec::new();
    let mut end_delimeter: Vec<Token> = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::NewLine(_) => {
                end_delimeter.push(tokens.next().unwrap());
                let second_newline = tokens.peek();
                match second_newline {
                    Some(Token::NewLine(_)) => {
                        end_delimeter.push(tokens.next().unwrap());
                        return Ok(Some(Symbol::Body {
                            text_tokens,
                            end_delimeter,
                        }));
                    }
                    Some(token) => {
                        return Err(SyntaxError::UnexpectedTokenError(token.clone()));
                    }
                    None => {
                        return Ok(Some(Symbol::Body {
                            text_tokens,
                            end_delimeter,
                        }));
                    }
                }
            }
            _ => {
                text_tokens.push(tokens.next().unwrap());
            }
        }
    }
    if text_tokens.len() > 0 {
        return Ok(Some(Symbol::Body {
            text_tokens,
            end_delimeter,
        }));
    }

    return Ok(None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::TokenGenerator;

    #[test]
    fn should_parse_body() {
        let mut tokens = TokenGenerator::new()
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
                assert_eq!(end_separator.len(), 2);
                assert_eq!(symbol.raw_value(), "this is a body\n\n");
                assert_eq!(symbol.value(), "this is a body");
            }
            _ => {
                panic!("Invalid symbol type");
            }
        }
    }
}
