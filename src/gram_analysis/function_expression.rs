use crate::{
    tokenizer::Token,
    types::{
        ASTType, BlockStatement, Body, Expression, FunctionExpression, GramAnalysisResult,
        Identifier,
    },
};

use super::block_statement::block_statement;

pub fn function_expression(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Expression>, &str> {
    if let Some(token) = tokens.get(cursor) {
        let mut ast = FunctionExpression {
            _type: ASTType::FunctionExpression,
            start: token.start,
            end: token.end,
            id: None,
            _async: false,
            expression: false,
            generator: false,
            params: Vec::new(),
            body: BlockStatement {
                _type: ASTType::BlockStatement,
                start: token.start,
                end: token.end,
                body: Vec::new(),
            },
        };
        if token._type.label == "name" {
            if let Some(next_token) = tokens.get(cursor + 1) {
                if next_token._type.label == "(" {
                    if let Some(next_next_token) = tokens.get(cursor + 2) {
                        if next_next_token._type.label == ")" {
                            if let Some(right_bracket_next_token) = tokens.get(cursor + 3) {
                                if right_bracket_next_token._type.label == "{" {
                                    if let Ok(block_statement_result) =
                                        block_statement(tokens, cursor + 2)
                                    {
                                        if let Body::BlockStatement(block_statement) =
                                            block_statement_result.ast
                                        {
                                            ast.body = block_statement;
                                            ast.end = block_statement_result.next_cursor
                                        }
                                    }
                                }
                            }
                        } else if next_next_token._type.label == "name" {
                            // 有参数
                            let mut current_cursor = cursor + 1;
                            let mut current_token = next_next_token;
                            loop {
                                let identifier = Identifier {
                                    _type: ASTType::Identifier,
                                    start: current_token.start,
                                    end: current_token.end,
                                    name: current_token.value.to_string(),
                                };
                                ast.params.push(identifier);
                                current_cursor += 1;
                                if let Some(param_next_token) = tokens.get(current_cursor) {
                                    if param_next_token._type.label == "," {
                                        current_cursor += 1;
                                        if let Some(comma_next_token) = tokens.get(current_cursor) {
                                            if comma_next_token._type.label == "name" {
                                                current_token = comma_next_token;
                                            } else {
                                                current_token = comma_next_token;
                                                break;
                                            }
                                        } else {
                                            return Err("");
                                        }
                                    } else {
                                        current_token = param_next_token;
                                        break;
                                    }
                                } else {
                                    return Err("");
                                }
                            }
                            if current_token._type.label == ")" {
                                if let Some(right_bracket_next_token) =
                                    tokens.get(current_cursor + 1)
                                {
                                    if right_bracket_next_token._type.label == "{" {
                                        if let Ok(block_statement_result) =
                                            block_statement(tokens, current_cursor + 1)
                                        {
                                            if let Body::BlockStatement(block_statement) =
                                                block_statement_result.ast
                                            {
                                                ast.body = block_statement;
                                                ast.end = block_statement_result.next_cursor
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if let Some(token) = tokens.get(cursor) {
            if token._type.label == "(" {
                if let Some(next_token) = tokens.get(cursor + 1) {
                    if next_token._type.label == ")" {
                        if let Some(right_bracket_next_token) = tokens.get(cursor + 2) {
                            if right_bracket_next_token._type.label == "{" {
                                if let Ok(block_statement_result) =
                                    block_statement(tokens, cursor + 2)
                                {
                                    if let Body::BlockStatement(block_statement) =
                                        block_statement_result.ast
                                    {
                                        ast.body = block_statement;
                                        ast.end = block_statement_result.next_cursor
                                    }
                                }
                            }
                        }
                    } else if next_token._type.label == "name" {
                        // 有参数
                        let mut current_cursor = cursor + 1;
                        let mut current_token = next_token;
                        loop {
                            let identifier = Identifier {
                                _type: ASTType::Identifier,
                                start: current_token.start,
                                end: current_token.end,
                                name: current_token.value.to_string(),
                            };
                            ast.params.push(identifier);
                            current_cursor += 1;
                            if let Some(param_next_token) = tokens.get(current_cursor) {
                                if param_next_token._type.label == "," {
                                    current_cursor += 1;
                                    if let Some(comma_next_token) = tokens.get(current_cursor) {
                                        if comma_next_token._type.label == "name" {
                                            current_token = comma_next_token;
                                        } else {
                                            current_token = comma_next_token;
                                            break;
                                        }
                                    } else {
                                        return Err("");
                                    }
                                } else {
                                    current_token = param_next_token;
                                    break;
                                }
                            } else {
                                return Err("");
                            }
                        }
                        if current_token._type.label == ")" {
                            if let Some(right_bracket_next_token) = tokens.get(current_cursor + 1) {
                                if right_bracket_next_token._type.label == "{" {
                                    if let Ok(block_statement_result) =
                                        block_statement(tokens, current_cursor + 1)
                                    {
                                        if let Body::BlockStatement(block_statement) =
                                            block_statement_result.ast
                                        {
                                            ast.body = block_statement;
                                            ast.end = block_statement_result.next_cursor
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if token._type.label == "async" {
            ast._async = true;
        } else if token._type.label == "*" {
            ast.generator = true;
        }
    }
    return Err("");
}
