use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

pub fn parse_description(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Symbol, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::Colon(_)) => {
            let next_token = tokens.peek();
            match next_token {
                Some(Token::Space(_)) => {
                    let start_delimeter = vec![current.unwrap(), tokens.next().unwrap()];
                    let mut text_tokens: Vec<Token> = Vec::new();
                    let mut end_delimiter: Vec<Token> = Vec::new();
                    while let Some(token) = tokens.peek() {
                        match token {
                            Token::NewLine(_) => {
                                end_delimiter.push(tokens.next().unwrap());
                                let second_newline = tokens.peek();
                                match second_newline {
                                    Some(Token::NewLine(_)) => {
                                        end_delimiter.push(tokens.next().unwrap());
                                        return Ok(Symbol::Description {
                                            start_delimeter,
                                            text_tokens,
                                            end_delimiter,
                                        });
                                    }
                                    Some(token) => {
                                        return Err(SyntaxError::UnexpectedTokenError(
                                            token.clone(),
                                        ))
                                    }
                                    None => {
                                        return Ok(Symbol::Description {
                                            start_delimeter,
                                            text_tokens,
                                            end_delimiter,
                                        })
                                    }
                                };
                            }
                            _ => text_tokens.push(tokens.next().unwrap()),
                        };
                    }
                    return Err(SyntaxError::UnexpectedEndOfFileError);
                }
                Some(token) => return Err(SyntaxError::UnexpectedTokenError(token.clone())),
                None => return Err(SyntaxError::UnexpectedEndOfFileError),
            }
        }
        Some(token) => return Err(SyntaxError::UnexpectedTokenError(token)),
        None => return Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{parser::types::Symbol, test_utils::TokenGenerator};

    use super::parse_description;
    #[test]
    fn should_parse_description() {
        let mut tokens = TokenGenerator::new()
            .colon()
            .space()
            .word("description")
            .space()
            .word("is")
            .space()
            .word("this")
            .newline()
            .newline()
            .generate_iter();
        let res = parse_description(&mut tokens);
        let symbol = res.unwrap();
        match &symbol {
            Symbol::Description {
                start_delimeter,
                text_tokens,
                end_delimiter,
            } => {
                assert_eq!(start_delimeter.len(), 2);
                assert_eq!(text_tokens.len(), 5);
                assert_eq!(end_delimiter.len(), 2);
                assert_eq!(symbol.raw_value(), ": description is this\n\n");
                assert_eq!(symbol.value(), "description is this");
            }
            _ => panic!("Error: {:?}", symbol),
        }
    }
}
