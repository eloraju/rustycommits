use crate::parser_lib::{
    errors::unexpected_token_error::UnexpectedTokenError,
    lexer::types::{enums::TokenType, token::Token},
    parser::{
        types::{
            enums::{ParserState, SymbolType},
            symbol::Symbol,
        },
        Parser,
    },
};

pub fn parse_type(
    parser: &mut Parser,
    current: &Token,
    next: &Token,
) -> Result<(), UnexpectedTokenError> {
    match current.token_type {
        TokenType::Word => {
            parser.symbols.push(Symbol {
                symbol_type: SymbolType::Type,
                tokens: vec![current.clone()],
            });

            match next.token_type {
                TokenType::ParenthesisOpen => {
                    parser.state = ParserState::Scope;
                    return Ok(());
                }

                TokenType::Colon => {
                    parser.state = ParserState::Description;
                    return Ok(());
                }

                _ => {
                    return Err(UnexpectedTokenError::new(next.clone()));
                }
            }
        }
        _ => Err(UnexpectedTokenError::new(current.clone())),
    }
}

#[cfg(test)]
mod tests {

    use crate::parser_lib::lexer::types::token::generate_word_token;

    use super::*;

    #[test]
    fn should_parse_type_with_colon_after() {
        let token = generate_word_token("feat");
        let ok_next_token = Token {
            token_type: TokenType::Colon,
            value: ":".into(),
            start_i: 4,
            end_i: 4,
            len: 1,
        };
        let mut parser = Parser::new();
        let result = parse_type(&mut parser, &token, &ok_next_token);
        assert!(result.is_ok());
        assert!(parser.symbols.len() == 1);
        assert!(parser.symbols[0].symbol_type == SymbolType::Type);
        assert!(parser.state == ParserState::Description);
    }
    #[test]
    fn should_return_error_with_space_after_type() {
        let token = generate_word_token("feat");
        let error_next_token = Token {
            token_type: TokenType::Word,
            value: " ".into(),
            start_i: 4,
            end_i: 4,
            len: 1,
        };
        let mut parser = Parser::new();
        let result = parse_type(&mut parser, &token, &error_next_token);
        assert!(result.is_err());
    }
}
