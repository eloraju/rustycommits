use thiserror::Error;

use super::lexer::types::Token;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Syntax error: Unexpected token '{0}' at index {}", .0.get_start_index())]
    UnexpectedTokenError(Token),
    #[error("Syntax error: Unexpted end of file")]
    UnexpectedEndOfFileError,
}
