use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::{enums::TokenType, Token},
    parser::{
        types::{enums::ParserState, Symbol},
        Parser,
    },
};

fn push_scope(parser: &mut Parser) {
    let end_delimeter = parser.delim_buffer.pop();
    let start_delimeter = parser.delim_buffer.pop();
    let text_token = parser.token_buffer.pop();
    match (start_delimeter, end_delimeter, text_token) {
        (Some(start_delimeter), Some(end_delimeter), Some(text_token)) => {
            parser.symbols.push(Symbol::Scope {
                start_delimeter,
                end_delimeter,
                text_token,
            });
        }
        _ => panic!(
            "We shouldn't ever get here! Scope is missing start or end delimeter or text token!"
        ),
    }
    parser.token_buffer.clear();
    parser.delim_buffer.clear();
}

pub fn parse_scope(
    parser: &mut Parser,
    current: &Token,
    next: Option<&Token>,
) -> Result<(), SyntaxError> {
    match current.token_type {
        TokenType::ParenthesisOpen => {
            parser.delim_buffer.push(current.clone());
            return Ok(());
        }

        TokenType::Word => {
            parser.token_buffer.push(current.clone());
            return match next {
                Some(next) => match next.token_type {
                    TokenType::ParenthesisClose => Ok(()),
                    _ => Err(SyntaxError::UnexpectedTokenError(next.clone())),
                },
                None => Err(SyntaxError::UnexpectedEndOfFileError(current.clone())),
            };
        }

        TokenType::ParenthesisClose => {
            if parser.token_buffer.len() == 0 {
                return Err(SyntaxError::UnexpectedTokenError(current.clone()));
            }

            parser.delim_buffer.push(current.clone());
            push_scope(parser);

            match next {
                Some(next) => match next.token_type {
                    TokenType::Colon => {
                        parser.state = ParserState::Description;
                        return Ok(());
                    }
                    _ => return Err(SyntaxError::UnexpectedTokenError(next.clone())),
                },
                None => return Err(SyntaxError::UnexpectedEndOfFileError(current.clone())),
            };
        }

        _ => return Err(SyntaxError::UnexpectedTokenError(current.clone())),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{
        lexer::types::enums::TokenType,
        parser::{
            parser_functions::parse_scope,
            types::{enums::ParserState, Symbol},
            Parser,
        },
        test_utils::{call_parser_function, TokenGenerator},
    };

    #[test]
    fn should_parse_scope_with_word() {
        let tokens = TokenGenerator::new()
            .add_special(TokenType::ParenthesisOpen)
            .add_word("scope")
            .add_special(TokenType::ParenthesisClose)
            .add_special(TokenType::Colon)
            .generate();
        let mut parser = Parser::new();
        parser.state = ParserState::Scope;
        let res = call_parser_function(tokens, parse_scope);
        assert!(res.is_err());
        let (_, symbols) = res.err().unwrap();
        assert_eq!(symbols.len(), 1);
        assert!(matches!(symbols[0], Symbol::Scope { .. }));
        assert_eq!(symbols[0].content_length(), 5);
        assert_eq!(symbols[0].total_length(), 7);
        assert_eq!(symbols[0].raw_value(), "(scope)");
        assert_eq!(symbols[0].value(), "scope");
    }

    #[test]
    fn should_return_error_when_missing_closing_parenthesis() {
        let tokens = TokenGenerator::new()
            .add_special(TokenType::ParenthesisOpen)
            .add_word("scope")
            .add_special(TokenType::Colon)
            .generate();

        let res = call_parser_function(tokens, parse_scope);
        assert!(res.is_err());
    }
}
