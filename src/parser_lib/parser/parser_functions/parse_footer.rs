use crate::parser_lib::{
    errors::SyntaxError,
    lexer::types::Token,
    parser::types::{Symbol, TokenIter},
};

use super::utils::{has_footer_start, take_until_newline_cond};

fn parse_key(tokens: &mut TokenIter) -> Result<Option<Vec<Token>>, SyntaxError> {
    if has_footer_start(tokens)? {
        let key = vec![
            tokens.next().unwrap(),
            tokens.next().unwrap(),
            tokens.next().unwrap(),
        ];
        return Ok(Some(key));
    }
    Ok(None)
}

fn parse_footer_text(tokens: &mut TokenIter) -> Result<Vec<Token>, SyntaxError> {
    take_until_newline_cond(tokens, |remaining| match remaining.peek() {
        Some(Token::Newline(_)) => Err(SyntaxError::expected_string(remaining.next().unwrap())),
        Some(_) => Ok(has_footer_start(remaining)?),
        None => Ok(true),
    })
}

// This runs after every new line
fn recurse_footers(tokens: &mut TokenIter) -> Result<Option<Vec<Symbol>>, SyntaxError> {
    match parse_key(tokens)? {
        None => Ok(None),
        Some(key) => {
            let footer = Symbol::Footer {
                start_delimiter: key,
                text_tokens: parse_footer_text(tokens)?,
            };

            let mut footers: Vec<Symbol> = Vec::new();
            footers.push(footer);

            if has_footer_start(tokens)? {
                if let Some(next_footer) = recurse_footers(tokens)? {
                    footers.extend(next_footer);
                }
            }

            Ok(Some(footers))
        }
    }
}

pub fn parse_footers(tokens: &mut TokenIter) -> Result<Option<Vec<Symbol>>, SyntaxError> {
    recurse_footers(tokens)
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::test_utils::TestTokenBuilder;

    use super::*;

    #[test]
    fn should_parse_single_footer() {
        let (mut tokens, _) = TestTokenBuilder::new()
            .word("footer")
            .colon()
            .space()
            .word("this")
            .generate_iter();
        let symbol = parse_footers(&mut tokens).unwrap().unwrap().pop().unwrap();
        assert!(matches!(symbol, Symbol::Footer { .. }));
        assert_eq!(symbol.no_delims_string(), "footer: this");
    }

    #[test]
    fn should_parse_multiple_footers() {
        let (mut tokens, _) = TestTokenBuilder::new()
            .word("footer")
            .colon()
            .space()
            .word("this")
            .newline()
            .word("another-footer")
            .space()
            .hash()
            .word("12")
            .generate_iter();
        let symbols = parse_footers(&mut tokens).unwrap().unwrap();
        assert_eq!(symbols.len(), 2);
        assert!(matches!(symbols[0], Symbol::Footer { .. }));
        assert!(matches!(symbols[1], Symbol::Footer { .. }));
        assert_eq!(symbols[0].no_delims_string(), "footer: this\n");
        assert_eq!(symbols[1].no_delims_string(), "another-footer #12");
    }
}
