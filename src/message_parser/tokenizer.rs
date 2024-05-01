use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Word(String),
    NewLine,
    Dash,
    Hash,
    Colon,
    Space,
    Underscore,
    OpenParenthesis,
    CloseParenthesis,
    Bang,
    EOF,
}

struct Tokenizer<'a> {
    message: &'a str,
    pub tokens: Vec<Token>,
    buffer: String,
    config: HashMap<char, Token>,
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
impl<'a> Tokenizer<'a> {
    pub fn new(message: &str) -> Tokenizer {
        let mut config: HashMap<char, Token> = HashMap::new();
        config.extend([
            ('\n', Token::NewLine),
            ('-', Token::Dash),
            ('#', Token::Hash),
            (':', Token::Colon),
            (' ', Token::Space),
            ('_', Token::Underscore),
            ('(', Token::OpenParenthesis),
            (')', Token::CloseParenthesis),
            ('!', Token::Bang),
        ]);
        Tokenizer {
            message,
            tokens: Vec::new(),
            buffer: String::new(),
            config,
        }
    }

    fn process(&mut self) {
        for c in self.message.chars() {
            match self.config.get(&c) {
                Some(token) => {
                    self.push_token(token.clone());
                }
                None => {
                    self.buffer.push(c);
                }
            }
        }
        self.push_token(Token::EOF);
    }

    fn push_token(&mut self, token: Token) {
        if !self.buffer.is_empty() {
            self.tokens.push(Token::Word(self.buffer.clone()));
            self.buffer.clear();
        }

        self.tokens.push(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_tokenize_simple_header() {
        let message = "feat: add new feature";
        let mut tokenizer = Tokenizer::new(message);
        tokenizer.process();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token::Word("feat".to_string()),
                Token::Colon,
                Token::Space,
                Token::Word("add".to_string()),
                Token::Space,
                Token::Word("new".to_string()),
                Token::Space,
                Token::Word("feature".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn should_tokenize_message_with_body() {
        let message = "feat: add new feature\n\nThis is the body of the message";
        let mut tokenizer = Tokenizer::new(message);
        tokenizer.process();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token::Word("feat".to_string()),
                Token::Colon,
                Token::Space,
                Token::Word("add".to_string()),
                Token::Space,
                Token::Word("new".to_string()),
                Token::Space,
                Token::Word("feature".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::Word("This".to_string()),
                Token::Space,
                Token::Word("is".to_string()),
                Token::Space,
                Token::Word("the".to_string()),
                Token::Space,
                Token::Word("body".to_string()),
                Token::Space,
                Token::Word("of".to_string()),
                Token::Space,
                Token::Word("the".to_string()),
                Token::Space,
                Token::Word("message".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn should_tokenize_message_with_body_and_footers() {
        let message = "feat!: add new feature\n\nThis is the body of the message\n\nBREAKING CHANGE: this is a breaking change";
        let mut tokenizer = Tokenizer::new(message);
        tokenizer.process();
        assert_eq!(
            tokenizer.tokens,
            vec![
                Token::Word("feat".to_string()),
                Token::Bang,
                Token::Colon,
                Token::Space,
                Token::Word("add".to_string()),
                Token::Space,
                Token::Word("new".to_string()),
                Token::Space,
                Token::Word("feature".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::Word("This".to_string()),
                Token::Space,
                Token::Word("is".to_string()),
                Token::Space,
                Token::Word("the".to_string()),
                Token::Space,
                Token::Word("body".to_string()),
                Token::Space,
                Token::Word("of".to_string()),
                Token::Space,
                Token::Word("the".to_string()),
                Token::Space,
                Token::Word("message".to_string()),
                Token::NewLine,
                Token::NewLine,
                Token::Word("BREAKING".to_string()),
                Token::Space,
                Token::Word("CHANGE".to_string()),
                Token::Colon,
                Token::Space,
                Token::Word("this".to_string()),
                Token::Space,
                Token::Word("is".to_string()),
                Token::Space,
                Token::Word("a".to_string()),
                Token::Space,
                Token::Word("breaking".to_string()),
                Token::Space,
                Token::Word("change".to_string()),
                Token::EOF,
            ]
        );
    }
}
