use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::{enums::TokenType, Token},
    parser::{
        types::{enums::ParserState, Symbol},
        Parser,
    },
};

fn push_body(parser: &mut Parser) {
    let end_separator = parser.delim_buffer.pop().unwrap();
    let start_separator = parser.delim_buffer.pop().unwrap();
    parser.symbols.push(Symbol::Body {
        text_tokens: parser.token_buffer.clone(),
        start_separator,
        end_separator,
    });
    parser.token_buffer.clear();
    parser.delim_buffer.clear();
}

pub fn parse_body(
    parser: &mut Parser,
    current: &Token,
    next: Option<&Token>,
) -> Result<(), SyntaxError> {
    match current.token_type {
        TokenType::NewLine => {
            // start delimeter
            if parser.token_buffer.len() == 0 && parser.delim_buffer.len() == 0 {
                parser.delim_buffer.push(current.clone());
                return Ok(());
            }

            match next {
                Some(next) => match next.token_type {
                    TokenType::NewLine => {
                        parser.delim_buffer.push(current.clone());
                        push_body(parser);
                        parser.state = ParserState::Footer;
                        return Ok(());
                    }
                    _ => {
                        parser.token_buffer.push(current.clone());
                        return Ok(());
                    }
                },
                None => {
                    parser.token_buffer.push(current.clone());
                    return Ok(());
                }
            }
        }
        _ => {
            parser.token_buffer.push(current.clone());
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::{call_parser_function, TokenGenerator};

    #[test]
    fn should_parse_body() {
        let tokens = TokenGenerator::new()
            .add_special(TokenType::NewLine)
            .add_word("this")
            .add_special(TokenType::Space)
            .add_word("is")
            .add_special(TokenType::Space)
            .add_word("a")
            .add_special(TokenType::Space)
            .add_word("body")
            .add_special(TokenType::NewLine)
            .add_special(TokenType::NewLine)
            .generate();

        let result = call_parser_function(tokens, parse_body);
        match result {
            Ok(symbols) => {
                assert_eq!(symbols.len(), 1);
                if let Some(symbol) = symbols.get(0) {
                    match symbol {
                        Symbol::Body {
                            text_tokens,
                            start_separator,
                            end_separator,
                        } => {
                            assert_eq!(text_tokens.len(), 7);
                            assert_eq!(start_separator.token_type, TokenType::NewLine);
                            assert_eq!(end_separator.token_type, TokenType::NewLine);
                            assert_eq!(symbol.raw_value(), "\nthis is a body\n");
                            assert_eq!(symbol.value(), "this is a body");
                        }
                        _ => {
                            panic!("Invalid symbol type");
                        }
                    }
                }
            }
            Err((error, _)) => {
                panic!("Error: {:?}", error);
            }
        }
    }
}
