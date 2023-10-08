use crate::{
    tokenizer::{MyOption, Token},
    types::{Body, GramAnalysisResult},
};

use super::{block_statement::block_statement, variable_statement::variable_statement};

pub fn statement(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Body>, &str> {
    if let Some(token) = tokens.get(cursor) {
        if let MyOption(keywordOption) = &token._type.keyword {
            if let Some(keyword) = keywordOption {
                if keyword == "{" {
                    return block_statement(tokens, cursor);
                } else if keyword == "const" || keyword == "let" || keyword == "var" {
                    return variable_statement(tokens, cursor);
                }
            }
        }
    }
    return Err("");
}
