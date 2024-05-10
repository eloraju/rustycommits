mod errors;
mod lexer;
mod parser;
mod slicable_rc_string;

pub use slicable_rc_string::SlicableRcString;
use std::rc::Rc;

use self::{errors::SyntaxError, lexer::Lexer, parser::types::CommitMessage};

pub struct CommitMessageParser {
    lexer: Lexer,
}

impl Default for CommitMessageParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CommitMessageParser {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(),
        }
    }

    pub fn parse(&mut self, commit_message: String) -> Result<CommitMessage, SyntaxError> {
        let message = Rc::new(commit_message);
        let tokens = self.lexer.process(&message);
        parser::Parser::process(tokens)
    }
}

#[cfg(test)]
pub mod test_utils;
