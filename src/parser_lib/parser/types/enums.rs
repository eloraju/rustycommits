#[derive(Debug, PartialEq)]
pub enum ParserState {
    Type,
    Scope,
    Description,
    BodyOrFooter,
    Body,
    Footer,
}
