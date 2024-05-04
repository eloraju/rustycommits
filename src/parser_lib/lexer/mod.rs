use std::collections::HashMap;

use self::types::{enums::TokenType, token::Token};

pub mod types;

pub struct Lexer {
    tokens: Vec<Token>,
    word_length: usize,
    special_chars: HashMap<char, TokenType>,
}

pub struct LexerResult {
    pub message: Box<str>,
    pub tokens: Vec<Token>,
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
        let mut special_chars: HashMap<char, TokenType> = HashMap::new();
        special_chars.extend([
            ('\n', TokenType::NewLine),
            ('-', TokenType::Dash),
            ('#', TokenType::Hash),
            (':', TokenType::Colon),
            (' ', TokenType::Space),
            ('_', TokenType::Underscore),
            ('(', TokenType::ParenthesisOpen),
            (')', TokenType::ParenthesisClose),
            ('!', TokenType::Bang),
            ('\0', TokenType::EOF),
        ]);
        Lexer {
            tokens: Vec::new(),
            word_length: 0,
            special_chars,
        }
    }

    fn process(&mut self, message: Box<str>) -> LexerResult {
        for (i, c) in message.char_indices() {
            dbg!(c);
            match self.special_chars.get(&c).cloned() {
                Some(token) => {
                    if self.word_length != 0 {
                        self.push_word(i - 1, &message);
                    }
                    self.push_token(token, i, &message);
                }
                None => {
                    self.word_length += 1;
                }
            };
        }
        if self.word_length != 0 {
            self.push_word(message.len() - 1, &message);
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            value: "".into(),
            start_i: message.len() - 1,
            end_i: message.len() - 1,
            len: 1,
        });

        let result = LexerResult {
            message,
            tokens: self.tokens.clone(),
        };

        self.reset();
        return result;
    }

    fn push_token(&mut self, token: TokenType, token_index: usize, message: &Box<str>) {
        dbg!(token, token_index);
        self.tokens.push(Token::new(
            token,
            message.get(token_index..token_index).unwrap().into(),
            token_index,
            token_index,
            1,
        ));
    }

    fn push_word(&mut self, word_end_index: usize, message: &Box<str>) {
        let word_start_index = word_end_index - (self.word_length - 1);
        let value: Box<str> = message
            .get(word_start_index..word_end_index + 1)
            .unwrap()
            .into();
        self.tokens.push(Token::new(
            TokenType::Word,
            value,
            word_start_index,
            word_end_index,
            self.word_length,
        ));
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
        let message = Box::<str>::from("test");
        let mut lexer = Lexer::new();
        let tokens = lexer.process(message).tokens;
        assert_eq!(tokens[0].token_type, TokenType::Word);
        assert_eq!(tokens[0].start_i, 0);
        assert_eq!(tokens[0].end_i, 3);
        assert_eq!(tokens[0].value, "test".into());
    }

    #[test]
    fn should_return_token_with_correct_indecies() {
        let message = Box::<str>::from("!");
        let mut lexer = Lexer::new();
        let tokens = lexer.process(message).tokens;
        assert_eq!(tokens[0].token_type, TokenType::Bang);
        assert_eq!(tokens[0].start_i, 0);
        assert_eq!(tokens[0].end_i, 0);
    }

    #[test]
    fn should_return_spaces_and_word_with_correct_indecies() {
        let message = Box::<str>::from("  test");
        let mut lexer = Lexer::new();
        let tokens = lexer.process(message).tokens;
        assert_eq!(tokens[0].token_type, TokenType::Space);
        assert_eq!(tokens[1].token_type, TokenType::Space);
        assert_eq!(tokens[2].token_type, TokenType::Word);
        assert_eq!(tokens[2].start_i, 2);
        assert_eq!(tokens[2].end_i, 5);
    }

    #[test]
    fn should_tokenize_simple_header() {
        let message = Box::<str>::from("feat: add new feature");
        let mut lexer = Lexer::new();
        let tokens = lexer.process(message).tokens;
        assert_eq!(
            tokens
                .iter()
                .map(|t| t.token_type)
                .collect::<Vec<TokenType>>(),
            vec![
                TokenType::Word,
                TokenType::Colon,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn should_tokenize_message_with_body() {
        let message = Box::<str>::from("feat: add new feature\n\nThis is the body of the message");
        let mut lexer = Lexer::new();
        let tokens = lexer.process(message).tokens;
        assert_eq!(
            tokens
                .iter()
                .map(|t| t.token_type)
                .collect::<Vec<TokenType>>(),
            vec![
                TokenType::Word,
                TokenType::Colon,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::NewLine,
                TokenType::NewLine,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn should_tokenize_message_with_body_and_footers() {
        let message = Box::<str>::from("feat!: add new feature\n\nThis is the body of the message\n\nBREAKING CHANGE: this is a breaking change\nThanks-to: @user");
        let mut lexer = Lexer::new();
        let tokens = lexer.process(message);
        assert_eq!(
            tokens
                .tokens
                .iter()
                .map(|t| t.token_type)
                .collect::<Vec<TokenType>>(),
            vec![
                TokenType::Word,
                TokenType::Bang,
                TokenType::Colon,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::NewLine,
                TokenType::NewLine,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::NewLine,
                TokenType::NewLine,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Colon,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::Space,
                TokenType::Word,
                TokenType::NewLine,
                TokenType::Word,
                TokenType::Dash,
                TokenType::Word,
                TokenType::Colon,
                TokenType::Space,
                TokenType::Word,
                TokenType::EOF,
            ]
        );
    }
}
