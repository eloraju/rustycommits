use std::borrow::Borrow;

use itertools::Itertools;

use self::{
    parser_functions::{parse_description, parse_scope, parse_type},
    types::{enums::ParserState, symbol::Symbol},
};

use super::{errors::unexpected_token_error::UnexpectedTokenError, lexer::types::token::Token};

mod parser_functions;
pub mod types;

pub struct Parser {
    state: ParserState,
    symbols: Vec<Symbol>,
    symbol_buff: Symbol,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: ParserState::Type,
            symbols: Vec::new(),
            symbol_buff: Symbol::default(),
        }
    }

    pub fn process(&mut self, tokens: Vec<Token>) -> Result<Vec<Symbol>, UnexpectedTokenError> {
        for (current, next) in tokens.iter().tuple_windows() {
            let res = match self.state {
                ParserState::Type => parse_type(self, current, next),
                ParserState::Scope => parse_scope(self, current, next),
                ParserState::Description => parse_description(self, current, next),
                _ => Ok(()),
                //ParserState::Body => self.parse_body(current, next),
                //ParserState::Footer => (),
                //ParserState::SyntaxError => break,
            };

            if let Err(e) = res {
                return Err(e);
            }
        }

        let parsed = self.symbols.clone();
        self.reset();
        return Ok(parsed);
    }

    fn reset(&mut self) {
        self.state = ParserState::Type;
        self.symbol_buff.clear();
        self.symbols.clear();
    }
}
