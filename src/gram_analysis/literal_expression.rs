use crate::{
    tokenizer::{Token, TokenValue},
    types::{ASTType, Expression, GramAnalysisResult, Literal, LiteralType, Regex},
};

pub fn literal_expression(
    tokens: &[Token],
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
                raw: &token_value.value.as_str(),
                regex: Some(Regex {
                    pattern: &token_value.pattern.as_str(),
                    flags: &token_value.flags.as_str(),
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
            raw: &token_value.as_str(),
            regex: None,
        });
        return Ok(GramAnalysisResult {
            ast,
            next_cursor: cursor + 1,
        });
    }
    return Err("");
}
