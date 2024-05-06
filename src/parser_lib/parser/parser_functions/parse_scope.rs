use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

pub fn parse_scope(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Symbol, SyntaxError> {
    let current = tokens.next();

    match current {
        Some(Token::ParenthesisOpen(_)) => {
            let expected_word = tokens.peek();
            match expected_word {
                Some(Token::Word(_)) => {
                    let text_token = tokens.next().unwrap();
                    let expected_closing_parenthesis = tokens.peek();
                    match expected_closing_parenthesis {
                        Some(Token::ParenthesisClose(_)) => {
                            let end_delimeter = tokens.next();
                            return Ok(Symbol::Scope {
                                start_delimeter: current.unwrap(),
                                end_delimeter: end_delimeter.unwrap(),
                                text_token,
                            });
                        }
                        Some(token) => {
                            return Err(SyntaxError::UnexpectedTokenError(token.clone()))
                        }
                        None => return Err(SyntaxError::UnexpectedEndOfFileError),
                    }
                }
                Some(unexpected) => {
                    return Err(SyntaxError::UnexpectedTokenError(unexpected.clone()))
                }
                None => return Err(SyntaxError::UnexpectedEndOfFileError),
            }
        }
        Some(token) => return Err(SyntaxError::UnexpectedTokenError(token)),
        None => return Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{
        parser::{parser_functions::parse_scope, types::Symbol},
        test_utils::TokenGenerator,
    };

    #[test]
    fn should_parse_scope_with_word() {
        let mut tokens = TokenGenerator::new()
            .parenthesis_open()
            .word("scope")
            .parenthesis_close()
            .generate_iter();
        let res = parse_scope(&mut tokens);
        let symbol = res.unwrap();
        assert!(matches!(symbol, Symbol::Scope { .. }));
        assert_eq!(symbol.content_length(), 5);
        assert_eq!(symbol.total_length(), 7);
        assert_eq!(symbol.raw_value(), "(scope)");
        assert_eq!(symbol.value(), "scope");
    }
}
