use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};
// Type is a single word, a bang is the only char we're interested in
pub fn parse_type(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Symbol, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::Word(_)) => {
            let next_token = tokens.peek();
            match next_token {
                Some(Token::Bang(_)) => {
                    let bang = tokens.next().unwrap();
                    return Ok(Symbol::Type {
                        text_token: current.unwrap(),
                        braking_change_token: Some(bang),
                    });
                }
                _ => {
                    return Ok(Symbol::Type {
                        text_token: current.unwrap(),
                        braking_change_token: None,
                    })
                }
            }
        }
        Some(token) => return Err(SyntaxError::UnexpectedTokenError(token)),
        None => return Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::parser_lib::test_utils::TokenGenerator;

    #[test]
    fn should_parse_type() {
        let mut tokens = TokenGenerator::new().word("feat").generate_iter();
        let res = parse_type(&mut tokens);
        let symbol = res.unwrap();
        assert!(matches!(symbol, Symbol::Type { .. }));
        assert_eq!(symbol.content_length(), 4);
        assert_eq!(symbol.raw_value(), "feat");
    }
}
