pub mod errors;
pub mod lexer;
pub mod parser;
mod slicable_rc_string;

pub use slicable_rc_string::SlicableRcString;

#[cfg(test)]
pub mod test_utils {
    use std::rc::Rc;

    use itertools::Itertools;

    use super::{
        lexer::types::{Token, WordDetails},
        parser::types::TokenIter,
        SlicableRcString,
    };

    enum TokenType {
        Word { start_index: usize, len: usize },
        Space(usize),
        Colon(usize),
        NewLine(usize),
        ParenthesisOpen(usize),
        ParenthesisClose(usize),
        Hash(usize),
    }
    pub struct TestTokens {
        test_token_buf: Vec<TokenType>,
        string: String,
    }

    impl TestTokens {
        pub fn new() -> TestTokens {
            TestTokens {
                test_token_buf: Vec::new(),
                string: String::new(),
            }
        }

        fn next_index(&self) -> usize {
            self.string.len()
        }

        pub fn word(&mut self, value: &str) -> &mut Self {
            let len = value.len();
            self.test_token_buf.push(TokenType::Word {
                start_index: self.next_index(),
                len,
            });
            self.string.push_str(value);
            return self;
        }

        pub fn space(&mut self) -> &mut Self {
            self.test_token_buf
                .push(TokenType::Space(self.next_index()));
            self.string.push(' ');
            return self;
        }

        pub fn hash(&mut self) -> &mut Self {
            self.test_token_buf.push(TokenType::Hash(self.next_index()));
            self.string.push('#');
            return self;
        }

        pub fn colon(&mut self) -> &mut Self {
            self.test_token_buf
                .push(TokenType::Colon(self.next_index()));
            self.string.push(':');
            return self;
        }

        pub fn newline(&mut self) -> &mut Self {
            self.test_token_buf
                .push(TokenType::NewLine(self.next_index()));
            self.string.push('\n');
            return self;
        }

        pub fn parenthesis_open(&mut self) -> &mut Self {
            self.test_token_buf
                .push(TokenType::ParenthesisOpen(self.next_index()));
            self.string.push('(');
            return self;
        }

        pub fn parenthesis_close(&mut self) -> &mut Self {
            self.test_token_buf
                .push(TokenType::ParenthesisClose(self.next_index()));
            self.string.push(')');
            return self;
        }

        pub fn generate(&mut self) -> Vec<Token> {
            let mut tokens: Vec<Token> = Vec::new();
            let slicable_rc_string = SlicableRcString::new(Rc::new(self.string.clone()));
            for token in self.test_token_buf.iter() {
                match token {
                    TokenType::Word { start_index, len } => {
                        tokens.push(Token::Word(WordDetails::new(
                            slicable_rc_string.substr(*start_index..*start_index + len),
                        )));
                    }
                    TokenType::Space(index) => {
                        tokens.push(Token::Space(slicable_rc_string.substr(*index..*index + 1)))
                    }
                    TokenType::Hash(index) => {
                        tokens.push(Token::Hash(slicable_rc_string.substr(*index..*index + 1)))
                    }
                    TokenType::Colon(index) => {
                        tokens.push(Token::Colon(slicable_rc_string.substr(*index..*index + 1)))
                    }
                    TokenType::NewLine(index) => tokens.push(Token::Newline(
                        slicable_rc_string.substr(*index..*index + 1),
                    )),
                    TokenType::ParenthesisClose(index) => tokens.push(Token::ParenthesisClose(
                        slicable_rc_string.substr(*index..*index + 1),
                    )),
                    TokenType::ParenthesisOpen(index) => tokens.push(Token::ParenthesisOpen(
                        slicable_rc_string.substr(*index..*index + 1),
                    )),
                }
            }
            return tokens;
        }

        pub fn generate_iter(&mut self) -> TokenIter {
            return self.generate().into_iter().multipeek();
        }
    }
}
