use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::Token,
    parser::types::{Symbol, TokenIter},
};

fn check_start_delimiter(tokens: &mut TokenIter) -> Result<Option<Token>, SyntaxError> {
    let current = tokens.peek();
    match current {
        Some(Token::ParenthesisOpen(_)) => Ok(tokens.next()),
        Some(Token::Colon(_)) => Ok(None),
        Some(_token) => Err(SyntaxError::UnexpectedTokenError(
            tokens.next().unwrap(),
            "'(' or ':'".to_string(),
        )),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

fn take_word(tokens: &mut TokenIter) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::Word(_)) => Ok(current.unwrap()),
        Some(token) => Err(SyntaxError::expected_string(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

fn check_end_delimiter(tokens: &mut TokenIter) -> Result<Token, SyntaxError> {
    let current = tokens.next();
    match current {
        Some(Token::ParenthesisClose(_)) => Ok(current.unwrap()),
        Some(token) => Err(SyntaxError::expected_parent_close(token)),
        None => Err(SyntaxError::UnexpectedEndOfFileError),
    }
}

pub fn parse_scope(tokens: &mut TokenIter) -> Result<Option<Symbol>, SyntaxError> {
    let start_delimiter = check_start_delimiter(tokens)?;

    if start_delimiter.is_none() {
        return Ok(None);
    }

    let word = take_word(tokens)?;
    let end_delimiter = check_end_delimiter(tokens)?;
    Ok(Some(Symbol::Scope {
        text_token: word,
        start_delimiter: start_delimiter.unwrap(),
        end_delimiter: end_delimiter,
    }))
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{
        parser::{parser_functions::parse_scope, types::Symbol},
        test_utils::TestTokenBuilder,
    };

    #[test]
    fn should_parse_scope_with_word() {
        let (mut tokens, _) = TestTokenBuilder::new()
            .parenthesis_open()
            .word("scope")
            .parenthesis_close()
            .generate_iter();
        let res = parse_scope(&mut tokens);
        let symbol = res.unwrap().unwrap();
        assert!(matches!(symbol, Symbol::Scope { .. }));
        assert_eq!(symbol.content_length(), 5);
        assert_eq!(symbol.total_length(), 7);
        assert_eq!(symbol.full_string(), "(scope)");
        assert_eq!(symbol.no_delims_string(), "scope");
    }
}
