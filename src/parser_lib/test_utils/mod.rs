mod test_token_builder;
pub use test_token_builder::{ExpectedValue, TestStrings, TestTokenBuilder};

use crate::parser_lib::parsing::types::CommitMessage;

use super::parsing::types::Symbol;
pub fn assert_commit_message_eq_expected(msg: CommitMessage, expected: TestStrings) {
    assert_part_value_eq_expected(msg.topic, expected.topic, "Topic");
    assert_part_value_eq_expected(msg.scope, expected.scope, "Scope");
    assert_part_value_eq_expected(msg.description, expected.description, "Description");
    assert_part_value_eq_expected(msg.body, expected.body, "Body");
    assert_footers_match(msg.footers, expected.footers);
}

fn assert_footers_match(actual: Option<Vec<Symbol>>, expected: Option<Vec<ExpectedValue>>) {
    match (&actual, &expected) {
        (Some(actual), Some(expected)) => {
            assert_eq!(
                actual.len(),
                expected.len(),
                "Footer count didn't match\nexpect = {}\nactual = {}",
                expected.len(),
                actual.len()
            );
            for (actual, expected) in actual.iter().zip(expected.iter()) {
                assert_part_value_eq_expected(
                    Some(actual.clone()),
                    Some(expected.clone()),
                    "Footer",
                );
            }
        }
        (None, None) => {}
        _ => panic!(
            "Footers should both be Some or None\nactual: \"{:?}\"\nexpected: \"{:?}\"",
            actual, expected
        ),
    }
}

fn assert_part_value_eq_expected(
    part: Option<Symbol>,
    expected: Option<ExpectedValue>,
    part_name: &str,
) {
    match (&part, &expected) {
        (Some(part), Some(expected)) => {
            assert!(
                part.content_string() == expected.no_delims,
                "{} (no delims) didn't match.\nexpect = \"{}\"\nactual = \"{}\"",
                part_name,
                expected.no_delims,
                part.content_string()
            );
            assert!(
                part.full_string() == expected.full,
                "{} full strings didn't match.\nexpect = \"{}\"\nactual = \"{}\"",
                part_name,
                expected.no_delims,
                part.full_string(),
            );
        }
        (None, None) => {}
        _ => panic!(
            "Part and expected value should both be Some \npart: \"{:?}\"\nexpected: \"{:?}\"",
            part, expected
        ),
    }
}
