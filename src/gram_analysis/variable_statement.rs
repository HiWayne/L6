use crate::tokenizer::{MyOption, Token};

use super::{
    variable_declaration::variable_declaration,
    variable_declaration_list::variable_declaration_list,
};

pub fn variable_statement(tokens: &'static [Token], cursor: usize) {
    if let Some(token) = tokens.get(cursor) {
        if let MyOption(keywordOption) = &token._type.keyword {
            if let Some(keyword) = keywordOption {
                if keyword == "let" || keyword == "var" {
                    variable_declaration_list(tokens, cursor);
                } else if keyword == "const" {
                    variable_declaration(tokens, cursor);
                }
            }
        }
    }
}
