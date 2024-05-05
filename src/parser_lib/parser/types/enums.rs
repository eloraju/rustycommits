#[derive(Debug, PartialEq)]
pub enum ParserState {
    Type,
    Scope,
    Description,
    BodyOrFooter,
    Body,
    Footer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Type,
    Scope,
    Description,
    Body,
    FooterKey,
    FooterValue,
    SectionDivider,
    BreakingChanges,
    None,
}
