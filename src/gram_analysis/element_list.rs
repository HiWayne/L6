use crate::{tokenizer::Token, types::Expression};

use super::expression::expression;

pub struct ElementListResult {
    pub elements: Vec<Expression>,
    pub next_cursor: usize,
}

pub fn element_list(tokens: &[Token], cursor: usize) -> Result<ElementListResult, &str> {
    let mut next_cursor = cursor;
    let mut elements = Vec::new();
    loop {
        let result = expression(tokens, next_cursor);
        if let Ok(value) = result {
            elements.push(value.ast);
            if let Some(token) = tokens.get(value.next_cursor) {
                if token._type.label == "," {
                    next_cursor = value.next_cursor + 1
                } else {
                    break;
                }
            } else {
                return Err("");
            }
        } else {
            break;
        }
    }
    return Ok(ElementListResult {
        elements,
        next_cursor,
    });
}
