use itertools::Itertools;

use crate::parser_lib::{errors::SyntaxError, lexing::types::Token};

use super::{
    parser_functions::{parse_body, parse_description, parse_footers, parse_scope, parse_topic},
    types::CommitMessage,
};

pub struct Parser {}
impl Parser {
    pub fn process(tokens: Vec<Token>) -> Result<CommitMessage, SyntaxError> {
        let mut tokens_iter = tokens.into_iter().multipeek();
        let topic = parse_topic(&mut tokens_iter)?;
        let scope = parse_scope(&mut tokens_iter)?;
        let description = parse_description(&mut tokens_iter)?;
        let body = parse_body(&mut tokens_iter)?;
        let footers = parse_footers(&mut tokens_iter)?;

        Ok(CommitMessage {
            topic: Some(topic),
            scope,
            description: Some(description),
            body,
            footers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::test_utils::{assert_commit_message_eq_expected, TestTokenBuilder};

    #[test]
    fn should_parse_simple_message() {
        let (tokens, expected) = TestTokenBuilder::new()
            .topic("feat")
            .scope("api")
            .description("added a new endpoint for users")
            .generate_vec();

        let parsed = Parser::process(tokens).unwrap();
        assert_commit_message_eq_expected(parsed, expected);
    }

    #[test]
    fn should_parse_message_with_body_and_footer() {
        let (tokens, expected) = TestTokenBuilder::new()
            .topic("feat")
            .scope("api")
            .description("added a new endpoint for users")
            .body(|builder| {
                builder
                    .string("this is the body of the commit message")
                    .newline()
            })
            .colon_footer("test: this is a footer", false)
            .generate_vec();

        let parsed = Parser::process(tokens).unwrap();
        assert_commit_message_eq_expected(parsed, expected)
    }

    #[test]
    fn should_parse_message_with_a_lot_of_stuff() {
        let (tokens, expected) = TestTokenBuilder::new()
            .topic("feat")
            .scope("api")
            .description_with_bang("added a new endpoint for users")
            .body(|builder| {
                builder
                    .string("this is the body of the commit message")
                    .newline()
                    .newline()
                    .newline()
                    .newline()
                    .string("this is the second line of the body")
                    .newline()
            })
            .colon_footer("test: this is a footer", true)
            .colon_footer("test2: this is another footer", true)
            .hash_footer("test3 #test this is yet another footer", true)
            .multi_line_footer(|builder| {
                builder
                    .hash_footer("test4 #test footer with a newline", true)
                    .string("this is still part of the last footer")
            })
            .generate_vec();

        let parsed = Parser::process(tokens).unwrap();
        assert_commit_message_eq_expected(parsed, expected)
    }
}
