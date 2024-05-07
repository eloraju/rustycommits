use std::rc::Rc;

use crate::parser_lib::SlicableRcString;

use super::types::{Token, WordDetails};

pub struct Lexer {
    tokens: Vec<Token>,
    word_length: usize,
    message: SlicableRcString,
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
            // TODO: Come up with a better way to initialize this
            message: SlicableRcString::new(Rc::new("empty".to_string())),
        }
    }

    fn process(&mut self, message: &Rc<String>) -> Vec<Token> {
        self.message = SlicableRcString::new(Rc::clone(message));
        for (i, c) in message.char_indices() {
            match c {
                '!' => self.push_token(Token::Bang(self.message.substr(i..i + 1)), i),
                ':' => self.push_token(Token::Colon(self.message.substr(i..i + 1)), i),
                '#' => self.push_token(Token::Hash(self.message.substr(i..i + 1)), i),
                '\n' => self.push_token(Token::NewLine(self.message.substr(i..i + 1)), i),
                ')' => self.push_token(Token::ParenthesisClose(self.message.substr(i..i + 1)), i),
                '(' => self.push_token(Token::ParenthesisOpen(self.message.substr(i..i + 1)), i),
                ' ' => self.push_token(Token::Space(self.message.substr(i..i + 1)), i),
                _ => self.word_length += 1,
            };
        }
        if self.word_length != 0 {
            self.push_word(self.message.len());
        }

        let result = self.tokens.clone();
        self.reset();
        return result;
    }

    fn push_token(&mut self, token: Token, index: usize) {
        if self.word_length != 0 {
            self.push_word(index);
        }
        self.tokens.push(token);
    }

    fn push_word(&mut self, end_i: usize) {
        let start_i = end_i - self.word_length;
        let string: SlicableRcString = self.message.substr(start_i..end_i);
        self.tokens.push(Token::Word(WordDetails::new(string)));
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
        let message = Rc::new("test".to_string());
        // closure to make all variables to out of scope -> references to message should be dropped
        {
            println!(
                "refs to message before test: {:?}",
                Rc::strong_count(&message)
            );
            let mut lexer = Lexer::new();
            let tokens = lexer.process(&message);

            if let Some(token) = tokens.get(0) {
                match token {
                    Token::Word(data) => {
                        println!(
                            "refs to message during test: {:?}",
                            Rc::strong_count(&message)
                        );

                        assert_eq!(data.value(), "test".to_string());
                        assert_eq!(data.start_index(), 0);
                        assert_eq!(data.end_index(), 4);
                        assert_eq!(data.len(), 4);
                    }
                    _ => panic!("Expected a word token"),
                }
            } else {
                panic!("Expected a token");
            }
        }

        println!(
            "refs to message at the end of test: {:?}",
            Rc::strong_count(&message)
        );
    }

    #[test]
    fn should_parse_simple_string() {
        let message = Rc::new("This is a string: with some (stuff)\n".to_string());
        {
            let mut lexer = Lexer::new();
            let tokens = lexer.process(&message);
            for token in tokens {
                println!("{:#?}", token);
            }
            assert!(true)
        }
    }
}
