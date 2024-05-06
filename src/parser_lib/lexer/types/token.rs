use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]

pub struct WordDetails {
    value: Rc<str>,
    start: usize,
    len: usize,
}

impl WordDetails {
    pub fn new(value: Rc<str>, start: usize, len: usize) -> WordDetails {
        WordDetails { value, start, len }
    }

    pub fn value(&self) -> String {
        self.value.to_string()
    }

    pub fn end_index(&self) -> usize {
        self.start + self.len - 1
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn start_index(&self) -> usize {
        self.start
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(WordDetails),
    Bang(usize),
    Colon(usize),
    Hash(usize),
    NewLine(usize),
    ParenthesisClose(usize),
    ParenthesisOpen(usize),
    Space(usize),
    None,
}

impl Token {
    fn get_value(&self) -> String {
        match self {
            Token::Word(token_data) => token_data.value(),
            Token::Bang(_) => "!".to_string(),
            Token::Colon(_) => ":".to_string(),
            Token::Hash(_) => "#".to_string(),
            Token::NewLine(_) => "\n".to_string(),
            Token::ParenthesisClose(_) => ")".to_string(),
            Token::ParenthesisOpen(_) => "(".to_string(),
            Token::Space(_) => " ".to_string(),
            Token::None => "[NONE]".to_string(),
        }
    }

    pub fn get_start_index(&self) -> usize {
        match self {
            Token::Word(token_data) => token_data.start_index(),
            Token::Bang(index) => *index,
            Token::Colon(index) => *index,
            Token::Hash(index) => *index,
            Token::NewLine(index) => *index,
            Token::ParenthesisClose(index) => *index,
            Token::ParenthesisOpen(index) => *index,
            Token::Space(index) => *index,
            Token::None => 0,
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            Token::Word(token_data) => token_data.len(),
            _ => 1,
        }
    }

    pub fn get_end_index(&self) -> usize {
        self.get_start_index() + self.get_length() - 1
    }

    pub fn get_variant_name(&self) -> String {
        match self {
            Token::Word(_) => "Word".to_string(),
            Token::Bang(_) => "Bang".to_string(),
            Token::Colon(_) => "Colon".to_string(),
            Token::Hash(_) => "Hash".to_string(),
            Token::NewLine(_) => "NewLine".to_string(),
            Token::ParenthesisClose(_) => "ParenthesisClose".to_string(),
            Token::ParenthesisOpen(_) => "ParenthesisOpen".to_string(),
            Token::Space(_) => "Space".to_string(),
            Token::None => "None".to_string(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", self.get_value()),
        }
    }
}
