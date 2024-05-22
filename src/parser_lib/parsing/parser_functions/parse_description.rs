use itertools::Itertools;

use crate::parser_lib::{
    errors::SyntaxError,
    lexing::types::Token,
    parsing::types::{Symbol, TokenIter},
};

fn check_bang(tokens: &mut TokenIter) -> Result<Option<Token>, SyntaxError> {
    let current = tokens.peek();
    match current {
        Some(Token::Bang(_)) => Ok(tokens.next()),
        Some(_) => Ok(None),
        None => Err(SyntaxError::UnexpectedEndOfFile),
    }
}

fn check_start_delimiter(tokens: &mut TokenIter) -> Result<Vec<Token>, SyntaxError> {
    let current = tokens.next();
    let next = tokens.peek();
    match (&current, next) {
        (Some(Token::Colon(_)), Some(Token::Space(_))) => {
            Ok(vec![current.unwrap(), tokens.next().unwrap()])
        }

        (Some(Token::Colon(_)), Some(_)) => {
            Err(SyntaxError::expected_space(tokens.next().unwrap()))
        }

        (Some(_), _) => Err(SyntaxError::expected_colon(current.unwrap())),
        (None, _) => Err(SyntaxError::UnexpectedEndOfFile),
    }
}

fn take_words(tokens: &mut TokenIter) -> Result<Vec<Token>, SyntaxError> {
    let next = tokens.peek();
    if next.is_none() {
        return Err(SyntaxError::UnexpectedEndOfFile);
    }

    Ok(tokens
        .take_while_ref(|token| !matches!(token, Token::Newline(_)))
        .collect_vec())
}

pub fn parse_description(tokens: &mut TokenIter) -> Result<Symbol, SyntaxError> {
    let braking_change_token = check_bang(tokens)?;
    let start_delimiter = check_start_delimiter(tokens)?;
    let text_tokens = take_words(tokens)?;

    Ok(Symbol::Description {
        start_delimiter,
        text_tokens,
        braking_change_token,
    })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::parser_lib::{parsing::types::Symbol, test_utils::TestTokenBuilder};

    use super::parse_description;
    #[test]
    fn should_parse_description() {
        let (tokens, _) = TestTokenBuilder::new()
            .colon()
            .space()
            .word("description")
            .space()
            .word("is")
            .space()
            .word("this")
            .newline()
            .newline()
            .generate_vec();
        dbg!(&tokens);
        let res = parse_description(&mut tokens.into_iter().multipeek());
        let symbol = res.unwrap();
        match &symbol {
            Symbol::Description {
                start_delimiter,
                text_tokens,
                braking_change_token: _,
            } => {
                assert_eq!(start_delimiter.len(), 2);
                assert_eq!(text_tokens.len(), 5);
                assert_eq!(symbol.full_string(), ": description is this");
                assert_eq!(symbol.content_string(), "description is this");
            }
            _ => panic!("Error: {:?}", symbol),
        }
    }

    #[test]
    fn should_parse_description_with_bang() {
        let (mut tokens, _) = TestTokenBuilder::new()
            .description_with_bang("description is this")
            .generate_iter();
        let res = parse_description(&mut tokens);
        let symbol = res.unwrap();
        assert_eq!(symbol.full_string(), "!: description is this");
    }
}
