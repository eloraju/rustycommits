use parser_lib::parser_lib::CommitMessageParser;
fn main() {
    let mut parser = CommitMessageParser::new();
    let commit_message = "test: test".to_string();
    let parsed = parser.parse(commit_message).unwrap();
    println!("{:?}", parsed);
}
