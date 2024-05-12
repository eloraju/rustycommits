use thiserror::Error;

use super::lexing::types::Token;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Syntax error: Unexpected token '{}' at index {}. Expected {1}.", .0.get_value(), .0.get_start_index())]
    UnexpectedToken(Token, String),
    #[error("Syntax error: Unexpted end of file")]
    UnexpectedEndOfFile,
}

impl SyntaxError {
    pub fn expected_newline(token: Token) -> Self {
        SyntaxError::UnexpectedToken(token, "'\n'".to_string())
    }

    pub fn expected_string(token: Token) -> Self {
        SyntaxError::UnexpectedToken(token, "a string".to_string())
    }

    pub fn expected_colon(token: Token) -> Self {
        SyntaxError::UnexpectedToken(token, "':'".to_string())
    }

    pub fn expected_space(token: Token) -> Self {
        SyntaxError::UnexpectedToken(token, "a space".to_string())
    }

    pub fn expected_parent_open(token: Token) -> Self {
        SyntaxError::UnexpectedToken(token, "'('".to_string())
    }

    pub fn expected_parent_close(token: Token) -> Self {
        SyntaxError::UnexpectedToken(token, "')'".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::TestTokenBuilder;
    #[test]
    fn should_parse_error_correctly() {
        let token = TestTokenBuilder::new()
            .word("test")
            .generate_vec()
            .0
            .pop()
            .unwrap();
        let error = SyntaxError::UnexpectedToken(token, "':'".to_string());
        assert_eq!(
            error.to_string(),
            "Syntax error: Unexpected token 'test' at index 0. Expected ':'."
        );
    }

    #[test]
    fn should_parse_error_index_correctly() {
        let (token, _) = TestTokenBuilder::new()
            .word("test")
            .colon()
            .word("value")
            .generate_vec();
        let error = SyntaxError::UnexpectedToken(token[2].clone(), "a space".to_string());
        assert_eq!(
            error.to_string(),
            "Syntax error: Unexpected token 'value' at index 5. Expected a space."
        );
    }

    #[test]
    fn should_display_newline_char_correctly() {
        let (token, _) = TestTokenBuilder::new().word("test").generate_vec();
        let error = SyntaxError::UnexpectedToken(token[0].clone(), "'\n'".to_string());
        assert_eq!(
            error.to_string(),
            "Syntax error: Unexpected token 'test' at index 0. Expected '\n'."
        );
    }
}
