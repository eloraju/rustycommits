pub mod errors;
pub mod lexer;
pub mod parser;

#[cfg(test)]
pub mod test_utils {
    use itertools::Itertools;

    use super::{
        errors::SyntaxError,
        lexer::types::{enums::TokenType, Token},
        parser::{types::Symbol, Parser},
    };

    fn get_token(token_type: TokenType, value: &str, start_i: usize) -> Token {
        Token {
            token_type,
            value: value.into(),
            start_i,
            end_i: start_i + value.len() - 1,
            len: value.len(),
        }
    }

    pub fn get_word_token(value: &str, start_i: usize) -> Token {
        return get_token(TokenType::Word, value, start_i);
    }

    pub fn get_special_token(token_type: TokenType, start_i: usize) -> Token {
        return get_token(token_type, &token_type.to_string(), start_i);
    }

    /**
     * Call a parser function with a list of tokens and return the result.
     * This bypasses the parsers internal state and allows for testing of individual parser functions.
     * Returns a tuple of the error and the symbols generated before the error if an error is encountered.
     */
    pub fn call_parser_function(
        tokens: Vec<Token>,
        parse_func: fn(&mut Parser, &Token, Option<&Token>) -> Result<(), SyntaxError>,
    ) -> Result<Vec<Symbol>, (SyntaxError, Vec<Symbol>)> {
        let mut parser = Parser::new();
        for (cur, next) in tokens
            .iter()
            .map(Some)
            .chain([None])
            .tuple_windows()
            .filter_map(|(c, n)| Some((c?, n)))
        {
            let res = parse_func(&mut parser, cur, next);
            match res {
                Err(e) => return Err((e, parser.symbols.clone())),
                _ => (),
            }
        }

        return Ok(parser.symbols.clone());
    }

    pub struct TokenGenerator {
        tokens: Vec<Token>,
    }

    impl From<Vec<Token>> for TokenGenerator {
        fn from(tokens: Vec<Token>) -> Self {
            TokenGenerator { tokens }
        }
    }

    impl TokenGenerator {
        pub fn new() -> TokenGenerator {
            TokenGenerator { tokens: Vec::new() }
        }

        pub fn add_word(&mut self, value: &str) -> &mut Self {
            self.tokens.push(get_word_token(value, self.next_index()));
            return self;
        }

        pub fn add_special(&mut self, token_type: TokenType) -> &mut Self {
            self.tokens
                .push(get_special_token(token_type, self.next_index()));
            return self;
        }

        pub fn generate(&mut self) -> Vec<Token> {
            return self.tokens.clone();
        }

        fn next_index(&self) -> usize {
            match self.tokens.last() {
                None => 0,
                _ => self.tokens.last().unwrap().end_i + 1,
            }
        }
    }
}
