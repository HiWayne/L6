use crate::tokenizer::{MyOption, Token};

use super::{block_statement::block_statement, variable_statement::variable_statement};

pub fn statement(tokens: &'static [Token], cursor: usize) {
    if let Some(token) = tokens.get(cursor) {
        if let MyOption(keywordOption) = &token._type.keyword {
            if let Some(keyword) = keywordOption {
                if keyword == "{" {
                    block_statement(tokens, cursor);
                } else if keyword == "const" || keyword == "let" || keyword == "var" {
                    variable_statement(tokens, cursor);
                }
            }
        }
    }
}
