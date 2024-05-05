use itertools::Itertools;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token};

use super::{
    parser_functions::{parse_description, parse_scope, parse_type},
    types::{enums::ParserState, Symbol},
};

pub struct Parser {
    pub state: ParserState,
    pub symbols: Vec<Symbol>,
    pub symbol_buff: Symbol,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: ParserState::Type,
            symbols: Vec::new(),
            symbol_buff: Symbol::default(),
        }
    }

    pub fn process(&mut self, tokens: Vec<Token>) -> Result<Vec<Symbol>, SyntaxError> {
        for (current, next) in tokens
            .iter()
            .map(Some)
            .chain([None])
            .tuple_windows()
            .filter_map(|(c, n)| Some((c?, n)))
        {
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
