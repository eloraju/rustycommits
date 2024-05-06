pub mod errors;
pub mod lexer;
pub mod parser;

#[cfg(test)]
pub mod test_utils {
    use std::vec::IntoIter;

    use itertools::{Itertools, MultiPeek};

    use super::lexer::types::{Token, WordDetails};
    pub struct TokenGenerator {
        tokens: Vec<Token>,
    }

    impl From<Vec<Token>> for TokenGenerator {
        fn from(tokens: Vec<Token>) -> Self {
            TokenGenerator { tokens }
        }
    }

    impl TokenGenerator {
        pub fn new() -> TokenGenerator {
            TokenGenerator { tokens: Vec::new() }
        }

        pub fn word(&mut self, value: &str) -> &mut Self {
            self.tokens.push(Token::Word(WordDetails::new(
                value.into(),
                self.next_index(),
                value.len(),
            )));
            return self;
        }

        pub fn space(&mut self) -> &mut Self {
            self.tokens.push(Token::Space(self.next_index()));
            return self;
        }

        pub fn colon(&mut self) -> &mut Self {
            self.tokens.push(Token::Colon(self.next_index()));
            return self;
        }

        pub fn newline(&mut self) -> &mut Self {
            self.tokens.push(Token::NewLine(self.next_index()));
            return self;
        }

        pub fn parenthesis_open(&mut self) -> &mut Self {
            self.tokens.push(Token::ParenthesisOpen(self.next_index()));
            return self;
        }

        pub fn parenthesis_close(&mut self) -> &mut Self {
            self.tokens.push(Token::ParenthesisClose(self.next_index()));
            return self;
        }

        pub fn generate(&mut self) -> Vec<Token> {
            return self.tokens.clone();
        }

        pub fn generate_iter(&mut self) -> MultiPeek<IntoIter<Token>> {
            return self.tokens.clone().into_iter().multipeek();
        }

        fn next_index(&self) -> usize {
            match self.tokens.last() {
                None => 0,
                _ => self.tokens.last().unwrap().get_end_index() + 1,
            }
        }
    }
}
