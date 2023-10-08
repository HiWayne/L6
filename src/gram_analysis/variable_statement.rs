use crate::{
    tokenizer::{MyOption, Token},
    types::{ASTType, Body, GramAnalysisResult, VariableDeclaration},
};

use super::{
    variable_declaration::variable_declaration,
    variable_declaration_list::variable_declaration_list,
};

pub fn variable_statement(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Body>, &str> {
    if let Some(token) = tokens.get(cursor) {
        let mut ast = VariableDeclaration {
            _type: ASTType::VariableDeclaration,
            start: token.start,
            end: token.end,
            declarations: Vec::new(),
        };
        if let MyOption(keywordOption) = &token._type.keyword {
            if let Some(keyword) = keywordOption {
                if keyword == "let" || keyword == "var" {
                    if let Ok(mut result) = variable_declaration_list(tokens, cursor) {
                        ast.declarations.append(&mut result.list);
                        ast.end = result.next_cursor - 1;
                        return Ok(GramAnalysisResult {
                            ast: Body::VariableDeclaration(ast),
                            next_cursor: result.next_cursor,
                        });
                    }
                } else if keyword == "const" {
                    if let Ok(result) = variable_declaration(tokens, cursor) {
                        ast.declarations.push(result.ast);
                        ast.end = result.next_cursor - 1;
                        return Ok(GramAnalysisResult {
                            ast: Body::VariableDeclaration(ast),
                            next_cursor: result.next_cursor,
                        });
                    }
                }
            }
        }
    }
    return Err("");
}
