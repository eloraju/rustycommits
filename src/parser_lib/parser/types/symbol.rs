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
    Topic {
        text_token: Token,
    },
    Scope {
        text_token: Token,
        start_delimiter: Token,
        end_delimiter: Token,
    },
    Description {
        text_tokens: Vec<Token>,
        start_delimiter: Vec<Token>,
        braking_change_token: Option<Token>,
    },
    Body {
        start_delimiter: Vec<Token>,
        text_tokens: Vec<Token>,
    },
    Footer {
        // Delimiter is either 'word: ' or 'word #word'
        start_delimiter: Vec<Token>,
        text_tokens: Vec<Token>,
    },
}

impl Symbol {
    pub fn get_content_tokens(&self) -> Vec<&Token> {
        match self {
            Symbol::Topic { text_token, .. } => vec![text_token],
            Symbol::Scope { text_token, .. } => vec![text_token],
            Symbol::Description { text_tokens, .. } => text_tokens.iter().collect(),
            Symbol::Body {
                text_tokens: tokens,
                ..
            } => tokens.iter().collect(),
            Symbol::Footer {
                start_delimiter: key,
                text_tokens,
            } => {
                let mut tokens: Vec<&Token> = Vec::new();
                tokens.extend(key);
                tokens.extend(text_tokens);
                tokens
            }
        }
    }

    pub fn get_all_tokens(&self) -> Vec<&Token> {
        match self {
            Symbol::Topic { text_token } => {
                vec![text_token]
            }

            Symbol::Scope {
                text_token,
                end_delimiter,
                start_delimiter,
            } => vec![start_delimiter, text_token, end_delimiter],

            Symbol::Description {
                text_tokens,
                start_delimiter,
                braking_change_token,
            } => {
                let mut tokens = Vec::new();
                if let Some(braking_token) = braking_change_token {
                    tokens.push(braking_token);
                }
                tokens.extend(start_delimiter);
                tokens.extend(text_tokens);
                tokens
            }
            Symbol::Body {
                start_delimiter,
                text_tokens,
            } => {
                let mut tokens: Vec<&Token> = Vec::new();
                tokens.extend(start_delimiter);
                tokens.extend(text_tokens);
                tokens
            }

            Symbol::Footer {
                start_delimiter: key,
                text_tokens: value,
            } => {
                let mut tokens: Vec<&Token> = Vec::new();
                tokens.extend(key);
                tokens.extend(value);
                tokens
            }
        }
    }
    pub fn full_string(&self) -> String {
        self.get_all_tokens().to_srcs().to_string()
    }

    pub fn no_delims_string(&self) -> String {
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

    pub fn delimiters(&self) -> Option<(Option<String>, Option<String>)> {
        match self {
            Symbol::Topic { .. } => None,
            Symbol::Scope {
                start_delimiter,
                end_delimiter,
                ..
            } => Some((
                Some(start_delimiter.get_value()),
                Some(end_delimiter.get_value()),
            )),
            Symbol::Description {
                start_delimiter, ..
            }
            | Symbol::Body {
                start_delimiter, ..
            }
            | Symbol::Footer {
                start_delimiter, ..
            } => Some((
                Some(start_delimiter.iter().map(|t| t.get_value()).collect()),
                None,
            )),
        }
    }
}
