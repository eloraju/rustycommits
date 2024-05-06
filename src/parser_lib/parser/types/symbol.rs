use itertools::Itertools;

use crate::parser_lib::lexer::types::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    // Use named fields instead of a tuple to make adding stuff later easier
    // e.g. word casing and delimiters
    Type {
        text_token: Token,
        braking_change_token: Option<Token>,
    },
    Scope {
        text_token: Token,
        start_delimeter: Token,
        end_delimeter: Token,
    },
    Description {
        text_tokens: Vec<Token>,
        start_delimeter: Vec<Token>,
        end_delimiter: Token,
    },
    Body {
        text_tokens: Vec<Token>,
        start_separator: Token,
        end_separator: Token,
    },
    Footer {
        tokens: Vec<Token>,
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
            Symbol::Footer { tokens } => tokens.iter().collect(),
        }
    }

    pub fn get_all_tokens(&self) -> Vec<&Token> {
        match self {
            Symbol::Type {
                braking_change_token,
                text_token,
            } => {
                if let Some(braking_token) = braking_change_token {
                    return vec![text_token, braking_token];
                } else {
                    return vec![text_token];
                }
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
            } => {
                let mut tokens = vec![];
                tokens.extend(start_delimeter);
                tokens.extend(text_tokens);
                tokens.push(end_delimiter);
                return tokens;
            }
            Symbol::Body {
                text_tokens,
                start_separator,
                end_separator,
            } => {
                let mut tokens = vec![start_separator];
                tokens.extend(text_tokens);
                tokens.push(end_separator);
                return tokens;
            }

            Symbol::Footer { tokens } => tokens.iter().collect(),
        }
    }
    pub fn raw_value(&self) -> String {
        match self {
            _ => self.get_all_tokens().iter().join(""),
        }
    }

    pub fn value(&self) -> String {
        match self {
            _ => self.get_content_tokens().iter().join(""),
        }
    }

    pub fn content_length(&self) -> usize {
        return self.get_content_tokens().iter().map(|t| t.len).sum();
    }

    pub fn total_length(&self) -> usize {
        return self.get_all_tokens().iter().map(|t| t.len).sum();
    }

    pub fn start_i(&self) -> usize {
        return self.get_all_tokens().first().unwrap().start_i;
    }

    pub fn end_i(&self) -> usize {
        return self.get_all_tokens().last().unwrap().end_i;
    }
}
