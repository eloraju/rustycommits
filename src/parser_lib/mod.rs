mod errors;
mod lexing;
mod parsing;
mod slicable_rc_string;

pub use slicable_rc_string::SlicableRcString;
use std::rc::Rc;

use self::{errors::SyntaxError, lexing::Lexer, parsing::types::CommitMessage};

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
        parsing::Parser::process(tokens)
    }
}

#[cfg(test)]
pub mod test_utils;
