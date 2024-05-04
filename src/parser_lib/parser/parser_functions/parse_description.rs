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

pub fn parse_description(
    parser: &mut Parser,
    current: &Token,
    next: &Token,
) -> Result<(), UnexpectedTokenError> {
    match current.token_type {
        TokenType::Colon => {
            parser.symbols.push(Symbol {
                symbol_type: SymbolType::SectionDivider,
                tokens: vec![current.clone()],
            });
            return Ok(());
        }
        TokenType::NewLine => match next.token_type {
            TokenType::NewLine => {
                parser.symbols.push(parser.symbol_buff.clone());
                parser.symbols.push(Symbol {
                    symbol_type: SymbolType::SectionDivider,
                    tokens: vec![current.clone()],
                });
                parser.symbol_buff.clear();

                parser.state = ParserState::Body;
                return Ok(());
            }
            _ => {
                // description must end with two newlines
                return Err(UnexpectedTokenError::new(next.clone()));
            }
        },
        _ => {
            parser.symbol_buff.tokens.push(current.clone());
            return Ok(());
        }
    }
}