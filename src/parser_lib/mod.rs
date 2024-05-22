mod errors;
mod lexing;
mod parsing;
mod slicable_rc_string;
mod validation;

pub use slicable_rc_string::SlicableRcString;
use std::rc::Rc;

use self::{
    errors::SyntaxError,
    lexing::Lexer,
    parsing::{types::CommitMessage, Parser},
    validation::{load_rules, Validator},
};

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

    pub fn process(&mut self, commit_message: String) -> Result<CommitMessage, SyntaxError> {
        let message = Rc::new(commit_message);
        let tokens = self.lexer.process(&message);
        let parsed = Parser::process(tokens)?;
        let _validation_result = Validator::validate(&parsed, load_rules());

        return Ok(parsed);
    }
}

#[cfg(test)]
pub mod test_utils;
