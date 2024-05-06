use std::rc::Rc;

use super::types::{Token, WordDetails};

pub struct Lexer {
    tokens: Vec<Token>,
    word_length: usize,
    message: Rc<str>,
}

/*
 * Meaningful symbols:
 * - Bang: '!'
 * - Colon: ':'
 * - Dash: '-'
 * - Hash: '#'
 * - NewLine: '\n'
 * - Space: ' '
 * - CloseParenthesis: ')'
 * - OpenParenthesis: '('
 */
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            word_length: 0,
            message: "".into(),
        }
    }

    fn process(&mut self, message: Rc<str>) -> Vec<Token> {
        self.message = message.clone().into();
        for (i, c) in message.char_indices() {
            match c {
                '!' => self.push_token(Token::Bang(i), i),
                ':' => self.push_token(Token::Colon(i), i),
                '#' => self.push_token(Token::Hash(i), i),
                '\n' => self.push_token(Token::NewLine(i), i),
                ')' => self.push_token(Token::ParenthesisClose(i), i),
                '(' => self.push_token(Token::ParenthesisOpen(i), i),
                ' ' => self.push_token(Token::Space(i), i),
                _ => self.word_length += 1,
            };
        }
        if self.word_length != 0 {
            self.push_word(self.message.len() - 1);
        }

        let result = self.tokens.clone();
        self.reset();
        return result;
    }

    fn push_token(&mut self, token: Token, index: usize) {
        if self.word_length != 0 {
            self.push_word(index - 1);
        }
        self.tokens.push(token);
    }

    fn push_word(&mut self, end_i: usize) {
        let start_i = end_i - (self.word_length - 1);
        let value: Rc<str> = self.message.get(start_i..end_i + 1).unwrap().into();
        self.tokens.push(Token::Word(WordDetails::new(
            value,
            start_i,
            self.word_length,
        )));
        self.word_length = 0;
    }

    fn reset(&mut self) {
        self.tokens.clear();
        self.word_length = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_word_with_correct_indecies() {
        let message = Rc::<str>::from("test");
        let mut lexer = Lexer::new();
        let tokens = lexer.process(message);
        if let Some(token) = tokens.get(0) {
            match token {
                Token::Word(data) => {
                    assert_eq!(data.value(), "test".to_string());
                    assert_eq!(data.start_index(), 0);
                    assert_eq!(data.end_index(), 3);
                    assert_eq!(data.len(), 4);
                }
                _ => panic!("Expected a word token"),
            }
        } else {
            panic!("Expected a token");
        }
    }
}
