use core::fmt;

use crate::parser_lib::lexer::types::token::Token;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnexpectedTokenError {
    pub token: Token,
}

impl UnexpectedTokenError {
    pub fn new(token: Token) -> UnexpectedTokenError {
        UnexpectedTokenError { token }
    }
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            let pretty = vec![
                format!(
                    "Syntax error: Unexpected token '{}' at index {}",
                    self.token, self.token.start_i
                ),
                format!("{:?}", self.token.token_type),
                format!("{:0$}{:^1$}", self.token.start_i + 1, self.token.len),
            ]
            .join("\n");
            write!(f, "{:?}", pretty)
        } else {
            write!(
                f,
                "Syntax error: Unexpected token at index {}",
                self.token.start_i
            )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser_lib::lexer::types::token::generate_word_token;

    use super::*;

    #[test]
    fn should_return_error_message() {
        let token = generate_word_token("test");
        let error = UnexpectedTokenError::new(token);
        assert_eq!(
            error.to_string(),
            "Syntax error: Unexpected token at index 0".to_string()
        );
    }
}
