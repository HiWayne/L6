use crate::{
    tokenizer::Token,
    types::{ASTType, GramAnalysisResult, VariableDeclarator},
};

use super::{expression::expression, identifier::identifier};

pub fn variable_declaration(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<VariableDeclarator>, &str> {
    if let Some(start_token) = tokens.get(cursor) {
        if let Ok(identifier_result) = identifier(tokens, cursor) {
            if let Some(lookaheadSymbol) = tokens.get(identifier_result.next_cursor) {
                if lookaheadSymbol._type.label == "=" {
                    if let Ok(expression_result) =
                        expression(tokens, identifier_result.next_cursor + 1)
                    {
                        if let Some(end_token) = tokens.get(expression_result.next_cursor - 1) {
                            let variable_declarator = VariableDeclarator {
                                _type: ASTType::VariableDeclarator,
                                start: start_token.start,
                                end: end_token.end,
                                id: identifier_result.ast,
                                init: expression_result.ast,
                            };
                            return Ok(GramAnalysisResult {
                                ast: variable_declarator,
                                next_cursor: expression_result.next_cursor,
                            });
                        }
                    }
                }
            }
        }
    }
    return Err("");
}
