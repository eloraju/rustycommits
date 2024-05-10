use crate::parser_lib::{lexer::types::Token, SlicableRcString};

trait SRcStringFromTokens {
    fn to_srcs(&self) -> SlicableRcString;
}

impl SRcStringFromTokens for Vec<&Token> {
    fn to_srcs(&self) -> SlicableRcString {
        let first = self.first().unwrap();
        let last = self.last().unwrap();
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
        braking_change_token: Option<Token>,
    },
    Body {
        start_delimeter: Vec<Token>,
        text_tokens: Vec<Token>,
    },
    Footer {
        // Key is either 'word: ' or 'word #word'
        key: Vec<Token>,
        text_tokens: Vec<Token>,
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
            Symbol::Footer { key, text_tokens } => {
                let mut tokens: Vec<&Token> = Vec::new();
                tokens.extend(key);
                tokens.extend(text_tokens);
                return tokens;
            }
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
                braking_change_token,
            } => {
                let mut tokens = Vec::new();
                if let Some(braking_token) = braking_change_token {
                    tokens.push(braking_token);
                }
                tokens.extend(start_delimeter);
                tokens.extend(text_tokens);
                tokens
            }
            Symbol::Body {
                start_delimeter,
                text_tokens,
            } => {
                let mut tokens: Vec<&Token> = Vec::new();
                tokens.extend(start_delimeter);
                tokens.extend(text_tokens);
                tokens
            }

            Symbol::Footer {
                key,
                text_tokens: value,
            } => {
                let mut tokens: Vec<&Token> = Vec::new();
                tokens.extend(key);
                tokens.extend(value);
                return tokens;
            }
        }
    }
    pub fn raw_value(&self) -> String {
        self.get_all_tokens().to_srcs().to_string()
    }

    pub fn value(&self) -> String {
        self.get_content_tokens().to_srcs().to_string()
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
