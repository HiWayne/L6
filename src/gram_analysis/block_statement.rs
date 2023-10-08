use crate::{
    tokenizer::Token,
    types::{ASTType, BlockStatement, Body, GramAnalysisResult},
};

use super::statement_list::statement_list;

pub fn block_statement(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Body>, &str> {
    if let Some(token) = tokens.get(cursor) {
        let mut ast = BlockStatement {
            _type: ASTType::BlockStatement,
            start: token.start,
            end: token.end,
            body: Vec::new(),
        };
        if let Ok(mut statement_list_result) = statement_list(tokens, cursor + 1) {
            ast.body.append(&mut statement_list_result.statements);
            return Ok(GramAnalysisResult {
                ast: Body::BlockStatement(ast),
                next_cursor: statement_list_result.next_cursor,
            });
        } else {
            return Err("");
        }
    } else {
        return Err("");
    }
}
