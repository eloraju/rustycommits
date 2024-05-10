mod commit_message;
pub mod enums;
mod symbol;

use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::lexer::types::Token;

pub use self::commit_message::CommitMessage;
pub use self::symbol::Symbol;

pub type TokenIter = MultiPeek<IntoIter<Token>>;
