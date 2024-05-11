mod parser_result;
mod symbol;

use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::lexer::types::Token;

pub use self::parser_result::CommitMessage;
pub use self::symbol::Symbol;

pub type TokenIter = MultiPeek<IntoIter<Token>>;
