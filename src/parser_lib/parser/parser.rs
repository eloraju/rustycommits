use itertools::Itertools;

use crate::parser_lib::{errors::SyntaxError, lexer::types::Token};

use super::{
    parser_functions::{parse_body, parse_description, parse_footers, parse_scope, parse_topic},
    types::CommitMessage,
};

pub fn process(tokens: Vec<Token>) -> Result<CommitMessage, SyntaxError> {
    let mut tokens_iter = tokens.into_iter().multipeek();
    let commit_type = parse_topic(&mut tokens_iter)?;
    let scope = parse_scope(&mut tokens_iter)?;
    let description = parse_description(&mut tokens_iter)?;
    let body = parse_body(&mut tokens_iter)?;
    let footer = parse_footers(&mut tokens_iter)?;

    Ok(CommitMessage {
        topic: Some(commit_type),
        scope,
        description: Some(description),
        body,
        footers: footer,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_lib::{
        parser::types::Symbol,
        test_utils::{assert_commit_message_eq_expected, ExpectedStrings, TestTokenBuilder},
    };

    #[test]
    fn should_parse_simple_message() {
        let (mut tokens, expected) = TestTokenBuilder::new()
            .topic("feat")
            .scope("api")
            .description("added a new endpoint for users")
            .generate_vec();

        let parsed = process(tokens).unwrap();
        assert_eq!(matches!(parsed.topic, Some(Symbol::Topic { .. })), true);
        assert_eq!(matches!(parsed.scope, Some(Symbol::Scope { .. })), true);
        assert_eq!(
            matches!(parsed.description, Some(Symbol::Description { .. })),
            true
        );
        assert_eq!(
            parsed.topic.unwrap().no_delims_string(),
            expected.topic.unwrap()
        );
        assert_eq!(
            parsed.scope.unwrap().no_delims_string(),
            expected.scope.unwrap().no_delims
        );
        assert_eq!(
            parsed.description.unwrap().no_delims_string(),
            expected.description.unwrap().no_delims
        );
    }

    #[test]
    fn should_parse_message_with_body_and_footer() {
        let (mut tokens, expected) = TestTokenBuilder::new()
            .topic("feat")
            .scope("api")
            .description("added a new endpoint for users")
            .body(|builder| {
                builder
                    .string("this is the body of the commit message")
                    .newline()
            })
            .colon_footer("test: this is a footer")
            .generate_vec();

        let parsed = process(tokens).unwrap();
        assert_commit_message_eq_expected(parsed, expected)
    }

    #[test]
    fn should_parse_message_with_a_lot_of_stuff() {
        let (mut tokens, expected) = TestTokenBuilder::new()
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
            .colon_footer("test: this is a footer")
            .colon_footer("test2: this is another footer")
            .hash_footer("test3 #test this is yet another footer")
            .multi_line_footer(|builder| {
                builder
                    .hash_footer("test4 #test footer with a newline")
                    .string("this is still part of the last footer")
            })
            .generate_vec();

        let parsed = process(tokens).unwrap();
        assert_commit_message_eq_expected(parsed, expected)
    }
}
