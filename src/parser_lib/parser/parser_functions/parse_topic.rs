use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::Token,
    parser::types::{Symbol, TokenIter},
};
// Type is a single word, a bang is the only char we're interested in
pub fn parse_topic(tokens: &mut TokenIter) -> Result<Symbol, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::Word(_)) => Ok(Symbol::Topic {
            text_token: current.unwrap(),
        }),
        Some(token) => Err(SyntaxError::expected_string(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::TestTokenBuilder;

    #[test]
    fn should_parse_type() {
        let (mut tokens, expected) = TestTokenBuilder::new().word("feat").generate_iter();
        let res = parse_topic(&mut tokens);
        let symbol = res.unwrap();
        assert!(matches!(symbol, Symbol::Topic { .. }));
        assert_eq!(symbol.content_length(), 4);
        assert_eq!(symbol.full_string(), "feat");
    }

    #[test]
    fn should_return_error() {
        let (mut tokens, _) = TestTokenBuilder::new().colon().generate_iter();
        let res = parse_topic(&mut tokens);
        assert!(matches!(res, Err(SyntaxError::UnexpectedTokenError(..))));
        assert_eq!(
            res.unwrap_err().to_string(),
            "Syntax error: Unexpected token ':' at index 0. Expected a string."
        )
    }
}
