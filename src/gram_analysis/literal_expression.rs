use crate::{
    tokenizer::{Token, TokenValue},
    types::{ASTType, Expression, GramAnalysisResult, Literal, LiteralType, Regex},
};

pub fn literal_expression(
    tokens: &'static [Token],
    cursor: usize,
    literal_type: LiteralType,
) -> Result<GramAnalysisResult<Expression>, &str> {
    let token = tokens.get(cursor).unwrap();
    if let LiteralType::REGEXP = literal_type {
        if let TokenValue::RegexpValue(token_value) = &token.value {
            let ast = Expression::Literal(Literal {
                _type: ASTType::Literal,
                start: token.start,
                end: token.end,
                value: literal_type,
                raw: &token_value.value,
                regex: Some(Regex {
                    pattern: &token_value.pattern,
                    flags: &token_value.flags,
                }),
            });
            return Ok(GramAnalysisResult {
                ast,
                next_cursor: cursor + 1,
            });
        }
    } else if let TokenValue::String(token_value) = &token.value {
        let ast = Expression::Literal(Literal {
            _type: ASTType::Literal,
            start: token.start,
            end: token.end,
            value: literal_type,
            raw: token_value,
            regex: None,
        });
        return Ok(GramAnalysisResult {
            ast,
            next_cursor: cursor + 1,
        });
    }
    return Err("");
}
