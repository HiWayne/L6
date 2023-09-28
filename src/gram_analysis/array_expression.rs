use crate::{
    tokenizer::{Token, TokenValue},
    types::{Expression, GramAnalysisResult, LiteralType},
};

use super::literal_expression::literal_expression;

pub fn array_expression(
    tokens: &'static [Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Expression>, &str> {
    
}
