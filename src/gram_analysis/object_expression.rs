use crate::{
    tokenizer::Token,
    types::{ASTType, ArrayExpression, Expression, GramAnalysisResult},
};

use super::object_property::object_property;

pub fn object_expression(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Expression>, &str> {
    if let Some(token) = tokens.get(cursor) {
        if token._type.label == "{" {
            object_property(tokens, cursor + 1);
        }
    }
    return Err("");
}
