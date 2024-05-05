use core::fmt;

use crate::parser_lib::lexer::types::Token;

#[derive(Debug, Clone)]
pub struct UnexpectedEndOfFileError {
    pub token: Token,
}
impl UnexpectedEndOfFileError {
    pub fn new(token: Token) -> UnexpectedEndOfFileError {
        UnexpectedEndOfFileError { token }
    }
}
impl fmt::Display for UnexpectedEndOfFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected end of file after '{}'", self.token.value)
    }
}
