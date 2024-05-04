use crate::parser_lib::{
    errors::unexpected_token_error::UnexpectedTokenError,
    lexer::types::{enums::TokenType, token::Token},
    parser::{
        types::enums::{ParserState, SymbolType},
        Parser,
    },
};

pub fn parse_scope(
    parser: &mut Parser,
    current: &Token,
    next: &Token,
) -> Result<(), UnexpectedTokenError> {
    match current.token_type {
        TokenType::ParenthesisOpen => {
            parser.symbol_buff.symbol_type = SymbolType::Scope;
            parser.symbol_buff.tokens.push(current.clone());
            return Ok(());
        }

        TokenType::Word => {
            parser.symbol_buff.tokens.push(current.clone());
            return match next.token_type {
                TokenType::ParenthesisClose => Ok(()),
                // Unclosed scope, no closing parenthesis. E.g. `feat(scope: description`
                _ => Err(UnexpectedTokenError::new(next.clone())),
            };
        }

        TokenType::ParenthesisClose => {
            if parser.symbol_buff.tokens.last().unwrap().token_type == TokenType::ParenthesisOpen {
                return Err(UnexpectedTokenError::new(current.clone()));
            }

            parser.symbol_buff.tokens.push(current.clone());
            parser.symbols.push(parser.symbol_buff.clone());
            parser.symbol_buff.clear();

            match next.token_type {
                TokenType::Colon => {
                    parser.state = ParserState::Description;
                    return Ok(());
                }
                _ => return Err(UnexpectedTokenError::new(next.clone())),
            };
        }

        _ => {
            return Err(UnexpectedTokenError::new(current.clone()));
        }
    }
}
