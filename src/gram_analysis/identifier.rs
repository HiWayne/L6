use crate::tokenizer::{Token, TokenValue};
use crate::types::{ASTType, GramAnalysisResult, Identifier, AST};

pub fn identifier(tokens: &[Token], cursor: usize) -> Result<GramAnalysisResult<Identifier>, &str> {
    if let Some(token) = tokens.get(cursor) {
        if let TokenValue::String(name) = &token.value {
            if token._type.label == "name" {
                let ast = Identifier {
                    _type: ASTType::Identifier,
                    start: token.start,
                    end: token.end,
                    name: String::from(name),
                };
                return Ok(GramAnalysisResult {
                    ast,
                    next_cursor: cursor + 1,
                });
            }
        }
    }
    return Err("occur error when parsing identifier");
}
