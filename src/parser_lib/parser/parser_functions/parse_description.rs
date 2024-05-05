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

pub fn parse_description(
    parser: &mut Parser,
    current: &Token,
    next: Option<&Token>,
) -> Result<(), SyntaxError> {
    match current.token_type {
        TokenType::Colon => {
            parser.symbols.push(Symbol {
                symbol_type: SymbolType::SectionDivider,
                tokens: vec![current.clone()],
            });
            return Ok(());
        }
        TokenType::NewLine => match next {
            Some(next) => match next.token_type {
                TokenType::NewLine | TokenType::EOF => {
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
                    // description must end with two newlines, no multiline descriptions allowed
                    return SyntaxError::unexpected_token(current.clone());
                }
            },
            None => Ok(()),
        },

        TokenType::EOF => {
            parser.symbols.push(parser.symbol_buff.clone());
            return Ok(());
        }
        _ => {
            parser.symbol_buff.tokens.push(current.clone());
            return Ok(());
        }
    }
}
