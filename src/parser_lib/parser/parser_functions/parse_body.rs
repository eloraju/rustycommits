use std::vec::IntoIter;

use itertools::{Itertools, MultiPeek};

use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::Token,
    parser::types::{Symbol, TokenIter},
};

use super::utils::{has_double_newline, has_footer_start, take_until_newline_cond};

pub fn parse_start_delimeter(tokens: &mut TokenIter) -> Option<Vec<Token>> {
    if has_double_newline(tokens) {
        Some(vec![tokens.next().unwrap(), tokens.next().unwrap()])
    } else {
        None
    }
}

fn check_end_of_body(tokens: &mut TokenIter) -> Result<bool, SyntaxError> {
    match tokens.peek() {
        Some(Token::Newline(_)) => match tokens.peek() {
            Some(_) => has_footer_start(tokens),
            None => Ok(false),
        },
        Some(_) => Ok(false),
        None => Ok(true),
    }
}

pub fn parse_body(tokens: &mut TokenIter) -> Result<Option<Symbol>, SyntaxError> {
    let start_delimeter = parse_start_delimeter(tokens);
    match (&start_delimeter, tokens.peek()) {
        (None, Some(toke)) => {
            return Err(SyntaxError::UnexpectedTokenError(
                toke.to_owned().clone(),
                "No newline before body".to_string(),
            ))
        }
        (None, None) => return Ok(None),
        (Some(_), _) => (),
    };
    let text_tokens = take_until_newline_cond(tokens, check_end_of_body)?;

    Ok(Some(Symbol::Body {
        start_delimeter: start_delimeter.unwrap(),
        text_tokens,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::TestTokens;

    #[test]
    fn should_parse_body() {
        let mut tokens = TestTokens::new()
            .newline()
            .newline()
            .word("this")
            .space()
            .word("is")
            .space()
            .word("a")
            .space()
            .word("body")
            .newline()
            .newline()
            .generate_iter();

        let result = parse_body(&mut tokens);
        let symbol = result.unwrap().unwrap();
        match &symbol {
            Symbol::Body {
                start_delimeter: _,
                text_tokens,
            } => {
                assert_eq!(text_tokens.len(), 9);
                assert_eq!(symbol.raw_value(), "\n\nthis is a body\n\n");
                assert_eq!(symbol.value(), "this is a body\n\n");
            }
            _ => {
                panic!("Invalid symbol type");
            }
        }
    }
}
