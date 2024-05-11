mod test_token_builder;
pub use test_token_builder::{ExpectedStrings, TestTokenBuilder};

use crate::parser_lib::parser::types::CommitMessage;

pub fn assert_commit_message_eq_expected(msg: CommitMessage, expected: ExpectedStrings) {
    let msg_no_delims = CommitMessage::to_no_delim_strings(msg.clone());
    let msg_full = CommitMessage::to_full_strings(msg);
    let expected_clone = expected.clone();
    let expected_no_delims = CommitMessageStr {
        topic: expected_clone.topic.map(|x| x.clone()),
        scope: expected_clone.scope.map(|x| x.no_delims.clone()),
        description: expected_clone.description.map(|x| x.no_delims.clone()),
        body: expected_clone.body.map(|x| x.no_delims.clone()),
        footers: expected_clone
            .footers
            .map(|x| x.into_iter().map(|x| x.clone()).collect()),
    };

    let expected_full = CommitMessageStr {
        topic: expected.topic.map(|x| x.clone()),
        scope: expected.scope.map(|x| x.full.clone()),
        description: expected.description.map(|x| x.full.clone()),
        body: expected.body.map(|x| x.full.clone()),
        footers: expected
            .footers
            .map(|x| x.into_iter().map(|x| x.clone()).collect()),
    };
    assert_eq!(
        msg_no_delims.topic, expected_no_delims.topic,
        "Topic should match"
    );
    assert_eq!(
        msg_no_delims.scope, expected_no_delims.scope,
        "Scope should match (no delims)"
    );
    assert_eq!(msg_full.scope, expected_full.scope, "Scope should match");
    assert_eq!(
        msg_no_delims.description, expected_no_delims.description,
        "Description should match (no delims)"
    );
    assert_eq!(
        msg_full.description, expected_full.description,
        "Description should match"
    );
    assert_eq!(
        msg_no_delims.body, expected_no_delims.body,
        "Body should match (no delims)"
    );
    assert_eq!(msg_full.body, expected_full.body, "Body should match ");
    assert_eq!(
        msg_no_delims.footers, expected_no_delims.footers,
        "Footers should match"
    );
}
