use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::{enums::TokenType, Token},
    parser::{
        types::{
            enums::{ParserState, SymbolType},
            Symbol,
        },
        Parser,
    },
};

pub fn parse_type(
    parser: &mut Parser,
    current: &Token,
    next: Option<&Token>,
) -> Result<(), SyntaxError> {
    match current.token_type {
        TokenType::Word => {
            parser.symbols.push(Symbol {
                symbol_type: SymbolType::Type,
                tokens: vec![current.clone()],
            });

            match next {
                Some(next) => match next.token_type {
                    TokenType::ParenthesisOpen => {
                        parser.state = ParserState::Scope;
                        return Ok(());
                    }

                    TokenType::Colon => {
                        parser.state = ParserState::Description;
                        return Ok(());
                    }

                    TokenType::Bang => {
                        return Ok(());
                    }

                    _ => SyntaxError::unexpected_token(next.clone()),
                },
                None => SyntaxError::end_of_file(current.clone()),
            }
        }

        TokenType::Bang => {
            if parser.symbols.is_empty() {
                return SyntaxError::unexpected_token(current.clone());
            }

            parser.symbols.push(Symbol {
                symbol_type: SymbolType::BreakingChanges,
                tokens: vec![current.clone()],
            });

            match next {
                Some(next) => match next.token_type {
                    TokenType::Colon => {
                        parser.state = ParserState::Description;
                        return Ok(());
                    }
                    _ => SyntaxError::unexpected_token(next.clone()),
                },
                None => return SyntaxError::end_of_file(current.clone()),
            }
        }
        _ => SyntaxError::unexpected_token(current.clone()),
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
        assert!(symbols[0].symbol_type == SymbolType::Type);
        assert!(symbols[0].len() == 4);
        assert!(symbols[0].value() == "feat");
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
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].symbol_type, SymbolType::Type);
        assert_eq!(symbols[1].symbol_type, SymbolType::BreakingChanges);
    }

    #[test]
    fn should_return_error_if_bang_now_followed_by_colon() {
        let tokens = TokenGenerator::new()
            .add_word("feat")
            .add_special(TokenType::Bang)
            .add_special(TokenType::ParenthesisOpen)
            .add_word("scope")
            .add_special(TokenType::ParenthesisClose)
            .generate();
        let result = call_parser_function(tokens, parse_type);
        assert!(result.is_err());
    }
}
