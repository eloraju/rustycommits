use super::types::{MessagePart, Token};

enum ParserState {
    Type,
    Scope,
    Description,
    Body,
    Footer,
}

struct ParsingResult {
    message_part: MessagePart,
    tokens: Vec<Token>,
    result: Result<(), String>,
}

struct Parser {
    tokens: Vec<Token>,
    state: ParserState,
    results: Vec<ParsingResult>,
}
