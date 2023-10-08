use crate::{
    tokenizer::Token,
    types::{ASTType, ArrayExpression, Expression, GramAnalysisResult},
};

use super::element_list::element_list;

pub fn array_expression(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Expression>, &str> {
    if let Some(token1) = tokens.get(cursor) {
        if let Some(token2) = tokens.get(cursor + 1) {
            let mut array_expression_ast = ArrayExpression {
                _type: ASTType::ArrayExpression,
                start: token1.start,
                end: token2.end,
                elements: Vec::new(),
            };
            if token2._type.label == "]" {
                return Ok(GramAnalysisResult {
                    ast: Expression::ArrayExpression(Some(Box::new(array_expression_ast))),
                    next_cursor: cursor + 2,
                });
            } else {
                if let Ok(element_list_result) = element_list(tokens, cursor + 1) {
                    if let Some(token) = tokens.get(element_list_result.next_cursor) {
                        if token._type.label == "]" {
                            array_expression_ast.end = token.end;
                            return Ok(GramAnalysisResult {
                                ast: Expression::ArrayExpression(Some(Box::new(
                                    array_expression_ast,
                                ))),
                                next_cursor: element_list_result.next_cursor + 1,
                            });
                        } else if token._type.label == "," {
                            if let Some(token) = tokens.get(element_list_result.next_cursor + 1) {
                                if token._type.label == "]" {
                                    return Ok(GramAnalysisResult {
                                        ast: Expression::ArrayExpression(Some(Box::new(
                                            array_expression_ast,
                                        ))),
                                        next_cursor: element_list_result.next_cursor + 2,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return Err("");
}
