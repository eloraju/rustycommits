use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token, parser::types::Symbol};

fn check_start_delimiter(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::ParenthesisOpen(_)) => Ok(current.unwrap()),
        Some(token) => Err(SyntaxError::UnexpectedTokenError(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

fn take_word(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::Word(_)) => Ok(current.unwrap()),
        Some(token) => Err(SyntaxError::UnexpectedTokenError(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

fn check_end_delimiter(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::ParenthesisClose(_)) => Ok(current.unwrap()),
        Some(token) => Err(SyntaxError::UnexpectedTokenError(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

pub fn parse_scope(tokens: &mut MultiPeek<IntoIter<Token>>) -> Result<Symbol, SyntaxError> {
    let start_delimiter = check_start_delimiter(tokens)?;
    let word = take_word(tokens)?;
    let end_delimiter = check_end_delimiter(tokens)?;
    return Ok(Symbol::Scope {
        text_token: word,
        start_delimeter: start_delimiter,
        end_delimeter: end_delimiter,
    });
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{
        parser::{parser_functions::parse_scope, types::Symbol},
        test_utils::TestTokens,
    };

    #[test]
    fn should_parse_scope_with_word() {
        let mut tokens = TestTokens::new()
            .parenthesis_open()
            .word("scope")
            .parenthesis_close()
            .generate_iter();
        let res = parse_scope(&mut tokens);
        let symbol = res.unwrap();
        assert!(matches!(symbol, Symbol::Scope { .. }));
        assert_eq!(symbol.content_length(), 5);
        assert_eq!(symbol.total_length(), 7);
        assert_eq!(symbol.raw_value(), "(scope)");
        assert_eq!(symbol.value(), "scope");
    }
}
