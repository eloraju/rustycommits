#[cfg(test)]
use std::rc::Rc;

use itertools::Itertools;

use crate::parser_lib::{
    lexer::types::{Token, WordDetails},
    parser::types::TokenIter,
    SlicableRcString,
};

#[derive(Debug, Clone)]
enum TokenType {
    Word { start_index: usize, len: usize },
    Bang(usize),
    Space(usize),
    Colon(usize),
    NewLine(usize),
    ParenthesisOpen(usize),
    ParenthesisClose(usize),
    Hash(usize),
}
pub struct TestTokenBuilder {
    test_token_buf: Vec<TokenType>,
    string: String,
    topic: Option<ExpectedValue>,
    scope: Option<ExpectedValue>,
    description: Option<ExpectedValue>,
    body: Option<ExpectedValue>,
    footers: Option<Vec<ExpectedValue>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExpectedValue {
    pub no_delims: String,
    pub full: String,
}

#[derive(Debug, Clone)]
pub struct TestStrings {
    pub topic: Option<ExpectedValue>,
    pub scope: Option<ExpectedValue>,
    pub description: Option<ExpectedValue>,
    pub body: Option<ExpectedValue>,
    pub footers: Option<Vec<ExpectedValue>>,
}
#[allow(dead_code)]
pub struct TestTokenBodyBuilder {
    test_token_buf: Vec<TokenType>,
    string: String,
}

impl TestTokenBodyBuilder {
    #[allow(dead_code)]
    pub fn new() -> TestTokenBodyBuilder {
        TestTokenBodyBuilder {
            test_token_buf: Vec::new(),
            string: String::new(),
        }
    }
}

impl TestTokenBuilder {
    pub fn new() -> TestTokenBuilder {
        TestTokenBuilder {
            test_token_buf: Vec::new(),
            string: String::new(),
            topic: None,
            scope: None,
            description: None,
            body: None,
            footers: None,
        }
    }

    fn body_builder(&self) -> TestTokenBuilder {
        TestTokenBuilder {
            test_token_buf: Vec::new(),
            string: self.string.clone(),
            topic: None,
            scope: None,
            description: None,
            body: None,
            footers: None,
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

    pub fn bang(&mut self) -> &mut Self {
        self.test_token_buf.push(TokenType::Bang(self.next_index()));
        self.string.push('!');
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

    pub fn string(&mut self, value: &str) -> &mut Self {
        let mut words = value.split_whitespace().multipeek();
        while let Some(word) = words.next() {
            self.word(word);
            if words.peek().is_some() {
                self.space();
            }
        }

        return self;
    }

    /// Alias for word(&mut self, value: &str)
    pub fn topic(&mut self, value: &str) -> &mut Self {
        self.topic = Some(ExpectedValue {
            no_delims: value.to_string(),
            full: value.to_string(),
        });
        self.word(value);
        return self;
    }

    pub fn scope(&mut self, value: &str) -> &mut Self {
        self.scope = Some(ExpectedValue {
            no_delims: value.to_string(),
            full: format!("({})", value),
        });
        self.parenthesis_open().word(value).parenthesis_close();
        return self;
    }

    pub fn description(&mut self, value: &str) -> &mut Self {
        self.description = Some(ExpectedValue {
            no_delims: value.to_string(),
            full: format!(": {}", value),
        });
        self.colon().space().string(value);
        return self;
    }

    pub fn description_with_bang(&mut self, value: &str) -> &mut Self {
        self.description = Some(ExpectedValue {
            no_delims: value.to_string(),
            full: format!("!: {}", value),
        });
        self.bang().colon().space().string(value);
        return self;
    }

    pub fn body(
        &mut self,
        build_fn: fn(builder: &mut TestTokenBuilder) -> &mut TestTokenBuilder,
    ) -> &mut Self {
        let mut body_builder = self.body_builder();
        body_builder.newline().newline();
        build_fn(&mut body_builder);
        self.test_token_buf.extend(body_builder.get_token_buf());
        let no_delims = body_builder.string.clone().split_off(self.string.len() + 2);
        self.body = Some(ExpectedValue {
            no_delims,
            full: format!(
                "\n\n{}",
                body_builder.string.clone().split_off(self.string.len() + 2)
            ),
        });
        self.string = body_builder.string;
        return self;
    }

    pub fn colon_footer(&mut self, input: &str, add_newline: bool) -> &mut Self {
        let value = if add_newline {
            format!("{}{}", input, "\n")
        } else {
            input.to_string()
        };

        if self.footers.is_none() {
            self.footers = Some(Vec::new());
        }
        let mut parts = value.split(": ");
        self.word(parts.next().unwrap())
            .colon()
            .space()
            .string(parts.next().unwrap());
        if add_newline {
            self.newline();
        }

        self.footers.as_mut().unwrap().push(ExpectedValue {
            full: format!("{}", value),
            no_delims: format!("{}", value),
        });
        return self;
    }

    pub fn hash_footer(&mut self, input: &str, add_newline: bool) -> &mut Self {
        let value = if add_newline {
            format!("{}{}", input, "\n")
        } else {
            input.to_string()
        };

        if self.footers.is_none() {
            self.footers = Some(Vec::new());
        }
        let mut parts = value.split(" #");
        self.word(parts.next().unwrap())
            .space()
            .hash()
            .string(parts.next().unwrap());
        if add_newline {
            self.newline();
        }
        self.footers.as_mut().unwrap().push(ExpectedValue {
            full: format!("{}", value),
            no_delims: format!("{}", value),
        });
        return self;
    }

    pub fn multi_line_footer(
        &mut self,
        builder_fn: fn(builder: &mut TestTokenBuilder) -> &mut TestTokenBuilder,
    ) -> &mut Self {
        if self.footers.is_none() {
            self.footers = Some(Vec::new());
        }

        let mut footer_builder = self.body_builder();
        builder_fn(&mut footer_builder);
        self.test_token_buf.extend(footer_builder.get_token_buf());
        let footer = footer_builder.string.clone().split_off(self.string.len());
        self.footers.as_mut().unwrap().push(ExpectedValue {
            full: footer.clone(),
            no_delims: footer.clone(),
        });
        self.string = footer_builder.string;
        return self;
    }

    fn get_token_buf(&self) -> Vec<TokenType> {
        self.test_token_buf.clone()
    }

    pub fn generate_vec(&mut self) -> (Vec<Token>, TestStrings) {
        let mut tokens: Vec<Token> = Vec::new();
        let slicable_rc_string = SlicableRcString::new(Rc::new(self.string.clone()));
        for token in self.test_token_buf.iter() {
            match token {
                TokenType::Word { start_index, len } => {
                    tokens.push(Token::Word(WordDetails::new(
                        slicable_rc_string.substr(*start_index..*start_index + len),
                    )));
                }
                TokenType::Bang(index) => {
                    tokens.push(Token::Bang(slicable_rc_string.substr(*index..*index + 1)))
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
        return (
            tokens,
            TestStrings {
                topic: self.topic.take(),
                scope: self.scope.take(),
                description: self.description.take(),
                body: self.body.take(),
                footers: self.footers.take(),
            },
        );
    }

    pub fn generate_iter(&mut self) -> (TokenIter, TestStrings) {
        let (tokens, expected) = self.generate_vec();
        return (tokens.into_iter().multipeek(), expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_correct_test_strings() {
        let (_, expected) = TestTokenBuilder::new()
            .topic("topic")
            .scope("scope")
            .description("description")
            .body(|builder| builder.string("this is a body").newline())
            .colon_footer("footer: footer", true)
            .hash_footer("footer #footer", true)
            .multi_line_footer(|builder| {
                builder
                    .colon_footer("multi: line", false)
                    .newline()
                    .string("footer")
            })
            .generate_vec();

        assert!(
            matches!(expected.topic, Some(topic) if topic.no_delims == "topic" && topic.full == "topic"),
            "Topic should match"
        );
        assert!(
            matches!(expected.scope, Some(scope) if scope.no_delims == "scope" && scope.full == "(scope)"),
            "Scope should match"
        );
        assert!(
            matches!(
                expected.description,
                Some(desc) if desc.no_delims == "description" && desc.full == ": description"
            ),
            "Description should match"
        );
        assert!(
            matches!(
                expected.body.clone(),
                Some(body) if body.no_delims == "this is a body\n"
            ),
            "Body should match (no delims)"
        );
        assert!(
            matches!(
                expected.body.clone(),
                Some(body) if  body.full == "\n\nthis is a body\n"
            ),
            "Full body didn't match\nExpected: \\n\\nthis is a body\\n\nGot: {}",
            expected.body.unwrap().full
        );

        assert_eq!(
            expected.footers,
            Some(vec![
                ExpectedValue {
                    no_delims: "footer: footer\n".to_string(),
                    full: "footer: footer\n".to_string()
                },
                ExpectedValue {
                    no_delims: "footer #footer\n".to_string(),
                    full: "footer #footer\n".to_string()
                },
                ExpectedValue {
                    no_delims: "multi: line\nfooter".to_string(),
                    full: "multi: line\nfooter".to_string()
                }
            ])
        );
    }
}
