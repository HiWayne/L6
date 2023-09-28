use crate::{
    tokenizer::{Token, TokenValue},
    types::{Expression, GramAnalysisResult, LiteralType},
};

use super::literal_expression::literal_expression;

pub fn element_list(
    tokens: &'static [Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Expression>, &str> {
}
