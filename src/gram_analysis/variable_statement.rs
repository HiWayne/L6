use crate::{
    tokenizer::{MyOption, Token},
    types::{ASTType, Body, DeclarationKind, GramAnalysisResult, VariableDeclaration},
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
        println!("variable_statement");
        let mut ast = VariableDeclaration {
            _type: ASTType::VariableDeclaration,
            start: token.start,
            end: token.end,
            declarations: Vec::new(),
            kind: DeclarationKind::Let,
        };
        if let MyOption(keywordOption) = &token._type.keyword {
            if let Some(keyword) = keywordOption {
                if keyword == "let" || keyword == "var" {
                    if keyword == "let" {
                        ast.kind = DeclarationKind::Let;
                    } else {
                        ast.kind = DeclarationKind::Var;
                    }
                    if let Ok(mut result) = variable_declaration_list(tokens, cursor + 1) {
                        ast.declarations.append(&mut result.list);
                        ast.end = result.next_cursor - 1;
                        if let Some(next_token) = tokens.get(result.next_cursor) {
                            if next_token._type.label == ";" {
                                return Ok(GramAnalysisResult {
                                    ast: Body::VariableDeclaration(ast),
                                    next_cursor: result.next_cursor + 1,
                                });
                            }
                        }
                        return Ok(GramAnalysisResult {
                            ast: Body::VariableDeclaration(ast),
                            next_cursor: result.next_cursor,
                        });
                    }
                } else if keyword == "const" {
                    ast.kind = DeclarationKind::Const;
                    if let Ok(result) = variable_declaration(tokens, cursor + 1) {
                        ast.declarations.push(result.ast);
                        ast.end = result.next_cursor - 1;
                        if let Some(next_token) = tokens.get(result.next_cursor) {
                            if next_token._type.label == ";" {
                                return Ok(GramAnalysisResult {
                                    ast: Body::VariableDeclaration(ast),
                                    next_cursor: result.next_cursor + 1,
                                });
                            }
                        }
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
