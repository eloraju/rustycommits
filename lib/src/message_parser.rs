use std::{iter::Peekable, str::Chars};

use crate::types::Token;

// 1. header
//  1.1. type
//  1.2. scope
//  1.3. description
// switch to body parsing after \n\n
// 2. body
// switch to footer parsin after first valid footer line
// 3. footer
//  3.1. breaking change
//  3.2. kebab-case: value

enum TokenizerState {
    Type,
    Scope,
    Description,
    Body,
    Footer,
}
struct Tokenizer<'a> {
    message: &'a str,
    state: TokenizerState,
    pub tokens: Vec<Token>,
    buffer: String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(message: &str) -> Tokenizer {
        Tokenizer {
            message,
            state: TokenizerState::Type,
            tokens: Vec::new(),
            buffer: String::new(),
        }
    }

    pub fn process(&mut self) -> &Vec<Token> {
        let mut chars = self.message.chars().peekable().into_iter();
        while let Some(char) = chars.next() {
            match self.state {
                TokenizerState::Type => self.tokenize_type(char),
                TokenizerState::Scope => self.tokenize_scope(char),
                TokenizerState::Description => self.tokenize_description(char),
                TokenizerState::Body => self.tokenize_body(char),
                TokenizerState::Footer => {
                    continue;
                }
            }
        }
        self.consume_buffer();

        return &self.tokens;
    }

    fn consume_buffer(&mut self) {
        match self.state {
            TokenizerState::Type => {
                self.tokens.push(Token::Type(self.buffer.clone()));
            }
            TokenizerState::Scope => {
                self.tokens.push(Token::Scope(self.buffer.clone()));
            }
            TokenizerState::Description => {
                self.tokens.push(Token::Description(self.buffer.clone()));
            }
            TokenizerState::Body => {
                self.tokens.push(Token::Paragraph(self.buffer.clone()));
            }
            TokenizerState::Footer => {
                self.tokens.push(Token::Footer(self.buffer.clone()));
            }
        }
        self.buffer.clear();
    }

    fn tokenize_type(&mut self, char: char) {
        match char {
            '(' => {
                self.consume_buffer();
                self.tokens.push(Token::OpenParenthesis);
                self.state = TokenizerState::Scope;
            }
            ')' => {
                self.consume_buffer();
                self.tokens.push(Token::CloseParenthesis);
            }
            ':' => {
                self.consume_buffer();
                self.tokens.push(Token::Colon);
                self.state = TokenizerState::Description;
            }
            _ => {
                self.buffer.push(char);
            }
        }
    }

    fn tokenize_scope(&mut self, char: char) {
        match char {
            ':' => {
                self.tokens.push(Token::Colon);
                self.state = TokenizerState::Description;
            }
            ')' => {
                self.consume_buffer();
                self.tokens.push(Token::CloseParenthesis);
            }
            _ => {
                self.buffer.push(char);
            }
        }
    }

    fn tokenize_description(&mut self, char: char) {
        match char {
            '\n' => {
                self.consume_buffer();
                self.tokens.push(Token::NewLine);
                self.state = TokenizerState::Body;
            }
            ' ' => {
                if self.tokens.last() == Some(&Token::Colon) {
                    self.tokens.push(Token::Space);
                } else {
                    self.buffer.push(char);
                }
            }
            _ => {
                self.buffer.push(char);
            }
        }
    }

    fn tokenize_body(&mut self, char: char) {
        match char {
            '\n' => {
                if self.tokens.last() == Some(&Token::NewLine) {
                    self.tokens.push(Token::NewLine);
                    self.state = TokenizerState::Footer;
                } else {
                    self.consume_buffer();
                    self.tokens.push(Token::NewLine);
                }
            }
            _ => {
                self.buffer.push(char);
            }
        }
    }

    fn tokenize_footer(&mut self, char: char) {
        match char {
            '\n' => {
                self.consume_buffer();
                self.tokens.push(Token::NewLine);
            }
            _ => {
                self.buffer.push(char);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_simple_header() {
        let message = "feat: add new feature";
        let mut tokenizer = Tokenizer::new(message);
        tokenizer.process();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token::Type("feat".to_string()),
                Token::Colon,
                Token::Space,
                Token::Description("add new feature".to_string())
            ]
        );
    }
    #[test]
    fn should_parse_header_with_type_and_scope() {
        let message = "feat(core): add new feature";
        let mut tokenizer = Tokenizer::new(message);
        tokenizer.process();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token::Type("feat".to_string()),
                Token::OpenParenthesis,
                Token::Scope("core".to_string()),
                Token::CloseParenthesis,
                Token::Colon,
                Token::Space,
                Token::Description("add new feature".to_string())
            ]
        );
    }
}
