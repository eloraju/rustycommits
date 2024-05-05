use itertools::Itertools;

use crate::parser_lib::lexer::types::Token;

use super::enums::SymbolType;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub tokens: Vec<Token>,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            symbol_type: SymbolType::None,
            tokens: Vec::new(),
        }
    }
}

impl Symbol {
    pub fn clear(&mut self) {
        self.symbol_type = SymbolType::None;
        self.tokens.clear();
    }

    pub fn value(&self) -> String {
        return self.tokens.iter().map(|t| t.value.as_ref()).join("");
    }

    pub fn len(&self) -> usize {
        return self.tokens.iter().map(|t| t.len).sum();
    }

    pub fn start_i(&self) -> usize {
        return self.tokens.first().unwrap().start_i;
    }

    pub fn end_i(&self) -> usize {
        return self.tokens.last().unwrap().end_i;
    }
}
