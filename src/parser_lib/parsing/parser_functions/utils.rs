use itertools::Itertools;

use crate::parser_lib::{errors::SyntaxError, lexing::types::Token, parsing::types::TokenIter};

pub fn has_footer_start(orig: &mut TokenIter) -> Result<bool, SyntaxError> {
    orig.reset_peek();
    let mut tokens = orig.clone();
    let t0 = tokens.next();
    let t1 = tokens.next();
    let t2 = tokens.next();
    let t3 = tokens.next();
    match (t0, t1, &t2, t3) {
        (Some(Token::Word(_)), Some(Token::Colon(_)), Some(Token::Space(_)), Some(_)) => Ok(true),

        (
            Some(Token::Word(_)),
            Some(Token::Space(_)),
            Some(Token::Hash(_)),
            Some(Token::Word(_)),
        ) => Ok(true),

        (Some(Token::Word(_)), Some(Token::Colon(_)), Some(_), _) => {
            Err(SyntaxError::expected_space(t2.unwrap()))
        }
        _ => Ok(false),
    }
}

pub type BreakoutFunction = fn(&mut TokenIter) -> Result<bool, SyntaxError>;

pub fn take_until_newline_cond(
    tokens: &mut TokenIter,
    breakout: BreakoutFunction,
) -> Result<Vec<Token>, SyntaxError> {
    let mut text_tokens: Vec<Token> = tokens
        .take_while_ref(|token| !matches!(token, Token::Newline(_)))
        .collect();

    let newline = tokens.next();

    match &newline {
        Some(Token::Newline(_)) => {
            text_tokens.push(newline.unwrap());
            if breakout(tokens)? {
                return Ok(text_tokens);
            }
            text_tokens.extend(take_until_newline_cond(tokens, breakout)?);
            Ok(text_tokens)
        }
        Some(token) => Err(SyntaxError::UnexpectedToken(
            token.to_owned().clone(),
            "this not to get here in the first place.".to_string(),
        )),
        None => Ok(text_tokens),
    }
}

pub fn has_double_newline(tokens: &mut TokenIter) -> bool {
    tokens.reset_peek();
    let result = matches!(tokens.peek(), Some(Token::Newline(_)))
        && matches!(tokens.peek(), Some(Token::Newline(_)));
    tokens.reset_peek();

    result
}
