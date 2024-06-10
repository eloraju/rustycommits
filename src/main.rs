use parser_lib::parser_lib::CommitMessageParser;
use std::io::{self, Read};
fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut commit_message = String::new();

    while let Ok(bytes) = stdin.read_to_string(&mut commit_message) {
        if bytes == 0 {
            break;
        }
    }

    let mut parser = CommitMessageParser::new();
    let parsed = parser.process(commit_message).unwrap();
    println!("{:?}", parsed);
}
