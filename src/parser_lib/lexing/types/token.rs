use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use crate::parser_lib::SlicableRcString;

#[derive(Debug, Clone, PartialEq)]

pub struct WordDetails {
    slicabe_rc_string: SlicableRcString,
}

impl WordDetails {
    pub fn new(value: SlicableRcString) -> WordDetails {
        WordDetails {
            slicabe_rc_string: value,
        }
    }

    pub fn value(&self) -> String {
        self.slicabe_rc_string.value()
    }

    pub fn end_index(&self) -> usize {
        self.slicabe_rc_string.end_index()
    }

    pub fn len(&self) -> usize {
        self.slicabe_rc_string.len()
    }

    pub fn start_index(&self) -> usize {
        self.slicabe_rc_string.start_index()
    }
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Word(WordDetails),
    Bang(SlicableRcString),
    Colon(SlicableRcString),
    Hash(SlicableRcString),
    Newline(SlicableRcString),
    ParenthesisClose(SlicableRcString),
    ParenthesisOpen(SlicableRcString),
    Space(SlicableRcString),
}

impl Token {
    fn get_srcs_ref(&self) -> &SlicableRcString {
        match self {
            Token::Word(token_data) => &token_data.slicabe_rc_string,
            Token::Bang(value) => value,
            Token::Colon(value) => value,
            Token::Hash(value) => value,
            Token::Newline(value) => value,
            Token::ParenthesisClose(value) => value,
            Token::ParenthesisOpen(value) => value,
            Token::Space(value) => value,
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            Token::Word(token_data) => token_data.value(),
            Token::Bang(value) => value.value(),
            Token::Colon(value) => value.value(),
            Token::Hash(value) => value.value(),
            Token::Newline(value) => value.value(),
            Token::ParenthesisClose(value) => value.value(),
            Token::ParenthesisOpen(value) => value.value(),
            Token::Space(value) => value.value(),
        }
    }

    pub fn get_start_index(&self) -> usize {
        match self {
            Token::Word(token_data) => token_data.start_index(),
            Token::Bang(value) => value.start_index(),
            Token::Colon(value) => value.start_index(),
            Token::Hash(value) => value.start_index(),
            Token::Newline(value) => value.start_index(),
            Token::ParenthesisClose(value) => value.start_index(),
            Token::ParenthesisOpen(value) => value.start_index(),
            Token::Space(value) => value.start_index(),
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            Token::Word(token_data) => token_data.len(),
            _ => 1,
        }
    }

    pub fn get_end_index(&self) -> usize {
        match self {
            Token::Word(token_data) => token_data.end_index(),
            Token::Bang(value) => value.end_index(),
            Token::Colon(value) => value.end_index(),
            Token::Hash(value) => value.end_index(),
            Token::Newline(value) => value.end_index(),
            Token::ParenthesisClose(value) => value.end_index(),
            Token::ParenthesisOpen(value) => value.end_index(),
            Token::Space(value) => value.end_index(),
        }
    }

    pub fn get_variant_name(&self) -> String {
        match self {
            Token::Word(_) => "Word".to_string(),
            Token::Bang(_) => "Bang".to_string(),
            Token::Colon(_) => "Colon".to_string(),
            Token::Hash(_) => "Hash".to_string(),
            Token::Newline(_) => "NewLine".to_string(),
            Token::ParenthesisClose(_) => "ParenthesisClose".to_string(),
            Token::ParenthesisOpen(_) => "ParenthesisOpen".to_string(),
            Token::Space(_) => "Space".to_string(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Token::Word(token_data) => token_data.len(),
            Token::Bang(value) => value.len(),
            Token::Colon(value) => value.len(),
            Token::Hash(value) => value.len(),
            Token::Newline(value) => value.end_index(),
            Token::ParenthesisClose(value) => value.len(),
            Token::ParenthesisOpen(value) => value.len(),
            Token::Space(value) => value.len(),
        }
    }

    pub fn get_super_slice(&self, span: Range<usize>) -> SlicableRcString {
        self.get_srcs_ref().super_slice(span)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Token::{} {{string: \"{}\"}}",
                self.get_variant_name(),
                self.get_value()
            )
        } else {
            write!(f, "Token::{}", self.get_variant_name())
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Token::{} {{string: \"{}\"}}",
                self.get_variant_name(),
                self.get_value()
            )
        } else {
            write!(f, "Token::{}", self.get_variant_name())
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.get_value()
    }
}
