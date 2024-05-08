use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};
// Type is a single word, a bang is the only char we're interested in
pub fn parse_type(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Symbol, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::Word(_)) => Ok(Symbol::Type {
            text_token: current.unwrap(),
        }),
        Some(token) => Err(SyntaxError::expected_string(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::TestTokens;

    #[test]
    fn should_parse_type() {
        let mut tokens = TestTokens::new().word("feat").generate_iter();
        let res = parse_type(&mut tokens);
        let symbol = res.unwrap();
        assert!(matches!(symbol, Symbol::Type { .. }));
        assert_eq!(symbol.content_length(), 4);
        assert_eq!(symbol.raw_value(), "feat");
    }

    #[test]
    fn should_return_error() {
        let mut tokens = TestTokens::new().colon().generate_iter();
        let res = parse_type(&mut tokens);
        assert!(matches!(res, Err(SyntaxError::UnexpectedTokenError(..))));
        assert_eq!(
            res.unwrap_err().to_string(),
            "Syntax error: Unexpected token ':' at index 0. Expected a string."
        )
    }
}
