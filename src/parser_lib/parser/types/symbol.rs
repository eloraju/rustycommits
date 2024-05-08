use crate::parser_lib::{lexer::types::Token, SlicableRcString};

trait SRcStringFromTokens {
    fn to_srcs(&self) -> SlicableRcString;
}

impl SRcStringFromTokens for Vec<&Token> {
    fn to_srcs(&self) -> SlicableRcString {
        let first = self.first().unwrap();
        let last = self.last().unwrap();
        dbg!(first, last);
        dbg!(first.get_start_index(), last.get_end_index());
        first.get_super_slice(first.get_start_index()..last.get_end_index())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FooterData {
    pub key: Token,
    pub value: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    // Use named fields instead of a tuple to make adding stuff later easier
    // e.g. word casing and delimiters
    Type {
        text_token: Token,
    },
    Scope {
        text_token: Token,
        start_delimeter: Token,
        end_delimeter: Token,
    },
    Description {
        text_tokens: Vec<Token>,
        start_delimeter: Vec<Token>,
        end_delimiter: Option<Vec<Token>>,
        braking_change_token: Option<Token>,
    },
    Body {
        text_tokens: Vec<Token>,
        end_delimeter: Option<Vec<Token>>,
    },
    Footer {
        key: Token,
        delimiter: Token,
        value: Token,
    },
}

impl Symbol {
    pub fn get_content_tokens(&self) -> Vec<&Token> {
        match self {
            Symbol::Type { text_token, .. } => vec![text_token],
            Symbol::Scope { text_token, .. } => vec![text_token],
            Symbol::Description { text_tokens, .. } => text_tokens.iter().collect(),
            Symbol::Body {
                text_tokens: tokens,
                ..
            } => tokens.iter().collect(),
            Symbol::Footer {
                key,
                delimiter,
                value,
            } => vec![key, delimiter, value],
        }
    }

    pub fn get_all_tokens(&self) -> Vec<&Token> {
        match self {
            Symbol::Type { text_token } => {
                vec![text_token]
            }

            Symbol::Scope {
                text_token,
                end_delimeter,
                start_delimeter,
            } => vec![start_delimeter, text_token, end_delimeter],

            Symbol::Description {
                text_tokens,
                start_delimeter,
                end_delimiter,
                braking_change_token,
            } => {
                let mut tokens = Vec::new();
                tokens.extend(start_delimeter);
                tokens.extend(text_tokens);
                if let Some(end_delimiter) = end_delimiter {
                    tokens.extend(end_delimiter);
                }
                if let Some(braking_token) = braking_change_token {
                    tokens.push(braking_token);
                }
                tokens
            }
            Symbol::Body {
                text_tokens,
                end_delimeter,
            } => {
                let mut tokens: Vec<&Token> = Vec::new();
                tokens.extend(text_tokens);
                if let Some(end_delimeter) = end_delimeter {
                    tokens.extend(end_delimeter);
                }
                tokens
            }

            Symbol::Footer {
                key,
                delimiter,
                value,
            } => vec![key, delimiter, value],
        }
    }
    pub fn raw_value(&self) -> SlicableRcString {
        self.get_all_tokens().to_srcs()
    }

    pub fn value(&self) -> SlicableRcString {
        self.get_content_tokens().to_srcs()
    }

    pub fn content_length(&self) -> usize {
        return self
            .get_content_tokens()
            .iter()
            .map(|t| t.get_length())
            .sum();
    }

    pub fn total_length(&self) -> usize {
        return self.get_all_tokens().iter().map(|t| t.get_length()).sum();
    }

    pub fn start_i(&self) -> usize {
        return self.get_all_tokens().first().unwrap().get_start_index();
    }

    pub fn end_i(&self) -> usize {
        return self.get_all_tokens().last().unwrap().get_end_index();
    }
}
