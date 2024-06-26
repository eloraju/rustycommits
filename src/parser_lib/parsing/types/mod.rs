mod parser_result;
mod symbol;

use std::vec::IntoIter;

use itertools::MultiPeek;

use crate::parser_lib::lexing::types::Token;

pub use self::parser_result::*;
pub use self::symbol::Symbol;

pub type TokenIter = MultiPeek<IntoIter<Token>>;
