use crate::gram_analysis::statement::statement;
use crate::tokenizer::Token;
use crate::types::{Body, GramAnalysisResult};

pub struct StatementListResult {
    pub statements: Vec<Body>,
    pub next_cursor: usize,
}

pub fn statement_list(
    tokens: &[Token],
    cursor: usize,
) -> Result<StatementListResult, &str> {
    let mut statements = Vec::new();
    let mut next_cursor = cursor;
    loop {
        match tokens.get(next_cursor) {
            Some(_) => {
                if let Ok(result) = statement(tokens, next_cursor) {
                    statements.push(result.ast);
                    next_cursor = result.next_cursor;
                } else {
                    return Err("");
                }
            }
            None => break,
        }
    }
    return Ok(StatementListResult {
        statements,
        next_cursor,
    });
}
