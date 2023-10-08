use crate::{
    tokenizer::Token,
    types::{Expression, GramAnalysisResult, LiteralType},
};

use super::{array_expression::array_expression, literal_expression::literal_expression};

pub fn expression(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Expression>, &str> {
    if let Some(token) = tokens.get(cursor) {
        // literal: number, string, boolean, null, undefined, regexp
        if token._type.label == "number" {
            return literal_expression(tokens, cursor, LiteralType::NUMBER);
        } else if token._type.label == "string" {
            return literal_expression(tokens, cursor, LiteralType::STRING);
        } else if token._type.label == "true" {
            return literal_expression(tokens, cursor, LiteralType::BOOLEAN);
        } else if token._type.label == "false" {
            return literal_expression(tokens, cursor, LiteralType::BOOLEAN);
        } else if token._type.label == "null" {
            return literal_expression(tokens, cursor, LiteralType::NULL);
        } else if token._type.label == "regexp" {
            return literal_expression(tokens, cursor, LiteralType::REGEXP);
        }
        // ArrayExpression
        else if token._type.label == "[" {
            return array_expression(tokens, cursor);
        }
    }
    return Err("");
}
