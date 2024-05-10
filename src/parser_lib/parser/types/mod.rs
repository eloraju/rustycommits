mod commit_message;
mod symbol;

use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::lexer::types::Token;

#[allow(unused_imports)]
pub use self::commit_message::{CommitMessage, CommitMessageStr};
pub use self::symbol::Symbol;

pub type TokenIter = MultiPeek<IntoIter<Token>>;
