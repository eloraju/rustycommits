mod unexpected_end_of_file;
mod unexpected_token_error;

use std::fmt;

use unexpected_end_of_file::UnexpectedEndOfFileError;
use unexpected_token_error::UnexpectedTokenError;

use super::lexer::types::Token;

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedTokenError(UnexpectedTokenError),
    UnexpectedEndOfFileError(UnexpectedEndOfFileError),
}

impl SyntaxError {
    pub fn unexpected_token(token: Token) -> Result<(), SyntaxError> {
        return Err(SyntaxError::UnexpectedTokenError(
            UnexpectedTokenError::new(token),
        ));
    }

    pub fn end_of_file(token: Token) -> Result<(), SyntaxError> {
        return Err(SyntaxError::UnexpectedEndOfFileError(
            UnexpectedEndOfFileError::new(token),
        ));
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxError::UnexpectedTokenError(e) => write!(f, "Syntax error: {}", e),
            SyntaxError::UnexpectedEndOfFileError(e) => write!(f, "Syntax error: {}", e),
        }
    }
}
