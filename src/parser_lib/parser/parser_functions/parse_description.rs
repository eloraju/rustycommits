use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::{enums::TokenType, Token},
    parser::{
        types::{enums::ParserState, Symbol},
        Parser,
    },
};

fn push_description(parser: &mut Parser) {
    let end_delimiter = parser.token_buffer.pop().unwrap();
    parser.symbols.push(Symbol::Description {
        text_tokens: parser.token_buffer.clone(),
        start_delimeter: parser.delim_buffer.clone(),
        end_delimiter,
    });
    parser.token_buffer.clear();
    parser.delim_buffer.clear();
}

pub fn parse_description(
    parser: &mut Parser,
    current: &Token,
    next: Option<&Token>,
) -> Result<(), SyntaxError> {
    match current.token_type {
        TokenType::Colon => match next {
            Some(next) => match next.token_type {
                TokenType::Space => {
                    parser.delim_buffer.push(current.clone());
                    return Ok(());
                }
                _ => return Err(SyntaxError::UnexpectedTokenError(next.clone())),
            },
            None => return Err(SyntaxError::UnexpectedEndOfFileError(current.clone())),
        },
        TokenType::NewLine => {
            match next {
                Some(next) => match next.token_type {
                    TokenType::NewLine => {
                        parser.token_buffer.push(current.clone());
                        push_description(parser);

                        parser.state = ParserState::Body;
                        return Ok(());
                    }
                    _ => {
                        // description must end with two newlines, no multiline descriptions allowed
                        return Err(SyntaxError::UnexpectedTokenError(current.clone()));
                    }
                },
                None => Ok(()),
            }
        }
        TokenType::Space => {
            if let Some(last_token) = parser.delim_buffer.last() {
                match last_token.token_type {
                    TokenType::Colon => {
                        parser.delim_buffer.push(current.clone());
                        return Ok(());
                    }
                    TokenType::Space => {
                        parser.token_buffer.push(current.clone());
                        return Ok(());
                    }
                    _ => return Err(SyntaxError::UnexpectedTokenError(last_token.clone())),
                }
            } else {
                return Err(SyntaxError::UnexpectedTokenError(current.clone()));
            }
        }
        _ => match next {
            Some(next) => match next.token_type {
                _ => {
                    parser.token_buffer.push(current.clone());
                    return Ok(());
                }
            },
            None => {
                parser.token_buffer.push(current.clone());
                push_description(parser);
                return Ok(());
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{
        lexer::types::enums::TokenType,
        parser::types::Symbol,
        test_utils::{call_parser_function, TokenGenerator},
    };

    use super::parse_description;
    #[test]
    fn should_parse_description() {
        let tokens = TokenGenerator::new()
            .add_special(TokenType::Colon)
            .add_special(TokenType::Space)
            .add_word("description")
            .add_special(TokenType::Space)
            .add_word("is")
            .add_special(TokenType::Space)
            .add_word("this")
            .add_special(TokenType::NewLine)
            .add_special(TokenType::NewLine)
            .generate();
        let res = call_parser_function(tokens, parse_description);
        match res {
            Ok(symbols) => {
                assert_eq!(symbols.len(), 1);
                if let Some(symbol) = symbols.get(0) {
                    match symbol {
                        Symbol::Description {
                            start_delimeter,
                            text_tokens,
                            end_delimiter,
                        } => {
                            assert_eq!(start_delimeter.len(), 2);
                            assert_eq!(text_tokens.len(), 5);
                            assert_eq!(symbol.raw_value(), ": description is this\n");
                            assert_eq!(symbol.value(), "description is this");
                        }
                        _ => panic!("Expected Symbol::Description"),
                    }
                } else {
                    panic!("Expected a symbol")
                }
            }
            Err(e) => panic!("{}", e.0),
        }
    }
}
