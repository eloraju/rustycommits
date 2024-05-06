use itertools::Itertools;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token};

use super::{
    parser_functions::{parse_body, parse_description, parse_scope, parse_type},
    types::{enums::ParserState, CommitMessage, Symbol},
};

pub struct Parser {
    pub state: ParserState,
    pub symbols: Vec<Symbol>,
    pub token_buffer: Vec<Token>,
    pub delim_buffer: Vec<Token>,
    pub result: CommitMessage,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: ParserState::Type,
            symbols: Vec::new(),
            token_buffer: Vec::new(),
            delim_buffer: Vec::new(),
            result: CommitMessage::default(),
        }
    }

    pub fn process(&mut self, tokens: Vec<Token>) -> Result<Vec<Symbol>, SyntaxError> {
        // This iterator returns a tuple of type (&Token, Option<&Token>).
        // Without this the tuple window would not return the last element
        // as current if there was an uneven amount of tokens in the vector
        for (current, next) in tokens
            // TODO: Check can we use into_iter instead and get rid of the .clone calls down the line
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
                ParserState::Body => parse_body(self, current, next),
                //ParserState::Footer => (),
                _ => Ok(()),
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
        self.token_buffer.clear();
        self.symbols.clear();
    }
}
