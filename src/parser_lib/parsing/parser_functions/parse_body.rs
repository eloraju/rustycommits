use crate::parser_lib::{
    errors::SyntaxError,
    lexing::types::Token,
    parsing::types::{Symbol, TokenIter},
};

use super::utils::{has_double_newline, has_footer_start, take_until_newline_cond};

pub fn parse_start_delimiter(tokens: &mut TokenIter) -> Option<Vec<Token>> {
    if has_double_newline(tokens) {
        Some(vec![tokens.next().unwrap(), tokens.next().unwrap()])
    } else {
        None
    }
}

fn check_end_of_body(tokens: &mut TokenIter) -> Result<bool, SyntaxError> {
    match tokens.peek() {
        Some(_) => has_footer_start(tokens),
        _ => Ok(false),
    }
}

pub fn parse_body(tokens: &mut TokenIter) -> Result<Option<Symbol>, SyntaxError> {
    let start_delimiter = parse_start_delimiter(tokens);
    match (&start_delimiter, tokens.peek()) {
        (None, Some(toke)) => {
            return Err(SyntaxError::UnexpectedToken(
                toke.to_owned().clone(),
                "No newline before body".to_string(),
            ))
        }
        (None, None) => return Ok(None),
        (Some(_), _) => (),
    };
    let text_tokens = take_until_newline_cond(tokens, check_end_of_body)?;

    Ok(Some(Symbol::Body {
        start_delimiter: start_delimiter.unwrap(),
        text_tokens,
    }))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::parser_lib::test_utils::TestTokenBuilder;

    use super::*;

    #[test]
    fn should_parse_body() {
        let (mut tokens, _) = TestTokenBuilder::new()
            .body(|builder| builder.string("this is a body"))
            .newline()
            .newline()
            .generate_iter();

        let result = parse_body(&mut tokens);
        let symbol = result.unwrap().unwrap();
        match &symbol {
            Symbol::Body {
                start_delimiter,
                text_tokens: _,
            } => {
                //assert_eq!(text_tokens.len(), 9);
                assert_eq!(start_delimiter.len(), 2);
                assert_eq!(
                    symbol.full_string(),
                    "\n\nthis is a body\n\n",
                    "symbol.full_string()"
                );
                assert_eq!(
                    symbol.content_string(),
                    "this is a body\n\n",
                    "symbol.no_delims_string()"
                );
            }
            _ => {
                panic!("Invalid symbol type");
            }
        }
    }

    #[test]
    fn should_parse_multi_line_body() {
        let (tokens, _) = TestTokenBuilder::new()
            .body(|builder| {
                builder
                    .string("this is a body")
                    .newline()
                    .space()
                    .string("with multiple lines")
                    .newline()
                    .newline()
                    .space()
                    .string("yupyup")
            })
            .generate_vec();

        let result = parse_body(&mut tokens.into_iter().multipeek());
        let symbol = result.unwrap().unwrap();
        let expected_str = "\n\nthis is a body\n with multiple lines\n\n yupyup".to_string();
        assert_eq!(symbol.full_string(), expected_str,);
    }

    #[test]
    fn should_leave_footer_alone() {
        let (mut tokens, _) = TestTokenBuilder::new()
            .body(|builder| builder.string("this is a body"))
            .newline()
            .colon_footer("test: this is a footer", true)
            .generate_iter();

        let result = parse_body(&mut tokens);
        let symbol = result.unwrap().unwrap();
        let left: Vec<Token> = tokens.collect();
        assert!(left.len() > 0);
        assert_eq!(left.len(), 11);
        assert_eq!(symbol.content_string(), "this is a body\n");
        assert_eq!(symbol.full_string(), "\n\nthis is a body\n");
        let left_str = left.iter().map(|t| t.get_value()).collect::<String>();
        assert_eq!(left_str, "test: this is a footer\n");
    }
}
