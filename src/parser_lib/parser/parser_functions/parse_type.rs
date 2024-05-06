use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::{enums::TokenType, Token},
    parser::{
        types::{enums::ParserState, Symbol},
        Parser,
    },
};

fn push_type(parser: &mut Parser, braking_change_token: Option<Token>) {
    parser.symbols.push(Symbol::Type {
        text_token: parser.token_buffer.pop().unwrap().clone(),
        braking_change_token,
    });
    parser.token_buffer.clear();
}

pub fn parse_type(
    parser: &mut Parser,
    current: &Token,
    next: Option<&Token>,
) -> Result<(), SyntaxError> {
    match current.token_type {
        TokenType::Word => {
            parser.token_buffer.push(current.clone());

            match next {
                Some(next) => match next.token_type {
                    TokenType::ParenthesisOpen => {
                        push_type(parser, None);
                        parser.state = ParserState::Scope;
                        return Ok(());
                    }

                    TokenType::Colon => {
                        push_type(parser, None);
                        parser.state = ParserState::Description;
                        return Ok(());
                    }

                    TokenType::Bang => {
                        return Ok(());
                    }

                    _ => Err(SyntaxError::UnexpectedTokenError(next.clone())),
                },
                None => Err(SyntaxError::UnexpectedEndOfFileError(current.clone())),
            }
        }

        TokenType::Bang => {
            if parser.token_buffer.is_empty() {
                return Err(SyntaxError::UnexpectedTokenError(current.clone()));
            }
            push_type(parser, Some(current.clone()));

            match next {
                Some(next) => match next.token_type {
                    TokenType::Colon => {
                        parser.state = ParserState::Description;
                        return Ok(());
                    }
                    _ => Err(SyntaxError::UnexpectedTokenError(next.clone())),
                },
                None => return Err(SyntaxError::UnexpectedEndOfFileError(current.clone())),
            }
        }
        _ => Err(SyntaxError::UnexpectedTokenError(current.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::{call_parser_function, TokenGenerator};

    #[test]
    fn should_parse_type_with_colon_after() {
        let tokens = TokenGenerator::new()
            .add_word("feat")
            .add_special(TokenType::Colon)
            .generate();
        let res = call_parser_function(tokens, parse_type);
        assert!(res.is_err());
        let (_, symbols) = res.err().unwrap();
        assert_eq!(symbols.len(), 1);
        assert!(matches!(symbols[0], Symbol::Type { .. }));
        assert_eq!(symbols[0].content_length(), 4);
        assert_eq!(symbols[0].raw_value(), "feat");
    }
    #[test]
    fn should_return_error_with_space_after_type() {
        let tokens = TokenGenerator::new()
            .add_word("feat")
            .add_special(TokenType::Space)
            .generate();
        let result = call_parser_function(tokens, parse_type);
        assert!(result.is_err());
    }

    #[test]
    fn should_detect_bang_correctly() {
        let tokens = TokenGenerator::new()
            .add_word("feat")
            .add_special(TokenType::Bang)
            .generate();
        let result = call_parser_function(tokens, parse_type);
        assert!(result.is_err());
        let (_, symbols) = result.err().unwrap();
        match &symbols[0] {
            Symbol::Type {
                braking_change_token,
                ..
            } => {
                if let Some(token) = braking_change_token {
                    assert_eq!(token.token_type, TokenType::Bang);
                } else {
                    panic!("Expected braking change token");
                }
            }
            _ => panic!("Wrong symbol type"),
        }
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].value(), "feat");
        assert_eq!(symbols[0].raw_value(), "feat!");
    }

    #[test]
    fn should_return_error_if_bang_not_followed_by_colon() {
        let tokens = TokenGenerator::new()
            .add_word("feat")
            .add_special(TokenType::Bang)
            .add_special(TokenType::ParenthesisOpen)
            .generate();
        let result = call_parser_function(tokens, parse_type);
        assert!(result.is_err());
    }
}
