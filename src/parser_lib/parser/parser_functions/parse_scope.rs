use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::{enums::TokenType, Token},
    parser::{
        types::enums::{ParserState, SymbolType},
        Parser,
    },
};

pub fn parse_scope(
    parser: &mut Parser,
    current: &Token,
    next: Option<&Token>,
) -> Result<(), SyntaxError> {
    match current.token_type {
        TokenType::ParenthesisOpen => {
            parser.symbol_buff.symbol_type = SymbolType::Scope;
            parser.symbol_buff.tokens.push(current.clone());
            return Ok(());
        }

        TokenType::Word => {
            parser.symbol_buff.tokens.push(current.clone());
            return match next {
                Some(next) => match next.token_type {
                    TokenType::ParenthesisClose => Ok(()),
                    _ => SyntaxError::unexpected_token(next.clone()),
                },
                None => SyntaxError::end_of_file(current.clone()),
            };
        }

        TokenType::ParenthesisClose => {
            if parser.symbol_buff.tokens.last().unwrap().token_type == TokenType::ParenthesisOpen {
                return SyntaxError::unexpected_token(current.clone());
            }

            parser.symbol_buff.tokens.push(current.clone());
            parser.symbols.push(parser.symbol_buff.clone());
            parser.symbol_buff.clear();

            match next {
                Some(next) => match next.token_type {
                    TokenType::Colon => {
                        parser.state = ParserState::Description;
                        return Ok(());
                    }
                    _ => return SyntaxError::unexpected_token(next.clone()),
                },
                None => return SyntaxError::end_of_file(current.clone()),
            };
        }

        _ => return SyntaxError::unexpected_token(current.clone()),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{
        lexer::types::enums::TokenType,
        parser::{
            parser_functions::parse_scope,
            types::enums::{ParserState, SymbolType},
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
        assert_eq!(symbols[0].symbol_type, SymbolType::Scope);
        assert_eq!(symbols[0].tokens.len(), 3);
        assert_eq!(symbols[0].value(), "(scope)");
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
