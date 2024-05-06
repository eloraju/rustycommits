use thiserror::Error;

use super::lexer::types::Token;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Syntax error: Unexpected token '{}' at index {}", .0.token_type, .0.start_i)]
    UnexpectedTokenError(Token),
    #[error("Syntax error: Unexpted end of file after '{0}'")]
    UnexpectedEndOfFileError(Token),
}
