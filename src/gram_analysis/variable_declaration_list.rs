use crate::{tokenizer::Token, types::VariableDeclarator};

use super::variable_declaration::variable_declaration;

pub struct VariableDeclarationListResult {
    pub list: Vec<VariableDeclarator>,
    pub next_cursor: usize,
}

pub fn variable_declaration_list(
    tokens: &[Token],
    cursor: usize,
) -> Result<VariableDeclarationListResult, &str> {
    let mut next_cursor = cursor;
    let mut list = Vec::new();
    loop {
        if let Ok(result) = variable_declaration(tokens, next_cursor) {
            next_cursor = result.next_cursor;
            list.push(result.ast);
            if let Some(token) = tokens.get(next_cursor) {
                if token._type.label == "," {
                    next_cursor += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            return Err("");
        }
    }
    return Ok(VariableDeclarationListResult { list, next_cursor });
}
