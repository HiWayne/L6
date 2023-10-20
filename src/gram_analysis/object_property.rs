use crate::{
    tokenizer::Token,
    types::{ASTType, GramAnalysisResult, Identifier, Kind, Property, PropertyKey},
};

use super::expression::expression;

pub fn object_property<'a>(
    tokens: &[Token],
    cursor: usize,
) -> Result<GramAnalysisResult<Property<'a>>, &'a str> {
    if let Some(token) = tokens.get(cursor) {
        let mut colon_cursor_index: usize = cursor;
        let mut key: PropertyKey;
        let mut kind: Kind = Kind::Init;
        let mut method = false;
        let mut function_start_cursor = cursor;
        if token._type.label == "name" {
            if let Some(next_token) = tokens.get(cursor + 1) {
                if next_token._type.label == "(" {
                    method = true;
                    function_start_cursor = cursor;
                } else {
                    colon_cursor_index = cursor + 1;
                }
                key = PropertyKey::Identifier(Identifier {
                    _type: ASTType::Identifier,
                    start: token.start,
                    end: token.end,
                    name: token.value.to_string(),
                });
            } else {
                return Err("");
            }
        } else if token._type.label == "string" {
            key = PropertyKey::Identifier(Identifier {
                _type: ASTType::Identifier,
                start: token.start,
                end: token.end,
                name: token.value.to_string(),
            });
            colon_cursor_index = cursor + 1;
        } else if token._type.label == "[" {
            if let Ok(expression_result) = expression(tokens, cursor + 1) {
                if let Some(next_token) = tokens.get(expression_result.next_cursor) {
                    if next_token._type.label != "]" {
                        return Err("");
                    } else {
                        key = PropertyKey::ComputedPropertyKey(expression_result.ast);
                        colon_cursor_index = expression_result.next_cursor + 1;
                        if let Some(right_bracket_next_token) =
                            tokens.get(expression_result.next_cursor + 1)
                        {
                            if right_bracket_next_token._type.label == "(" {
                                method = true;
                                function_start_cursor = expression_result.next_cursor + 1;
                            }
                        }
                    }
                } else {
                    return Err("");
                }
            } else {
                return Err("");
            }
        } else if token._type.label == "get" || token._type.label == "set" {
            if token._type.label == "get" {
                kind = Kind::Get;
            } else {
                kind = Kind::Set;
            }
            if let Some(next_token) = tokens.get(cursor + 1) {
                if next_token._type.label == "name" {
                    key = PropertyKey::Identifier(Identifier {
                        _type: ASTType::Identifier,
                        start: next_token.start,
                        end: next_token.end,
                        name: next_token.value.to_string(),
                    });
                    if let Some(identifier_next_token) = tokens.get(cursor + 2) {
                        if identifier_next_token._type.label == "(" {
                            function_start_cursor = cursor + 1;
                        } else {
                            return Err("");
                        }
                    }
                } else {
                    return Err("");
                }
            } else {
                return Err("");
            }
        } else {
            return Err("");
        }
        if let Kind::Init = kind {
            if !method {
                if let Some(token) = tokens.get(colon_cursor_index) {
                    if token._type.label == ":" {
                        if let Ok(expression_result) = expression(tokens, colon_cursor_index + 1) {
                            let property_ast = Property {
                                _type: ASTType::Property,
                                start: token.start,
                                end: token.end,
                                method,
                                shorthand: false,
                                computed: false,
                                key,
                                value: expression_result.ast,
                                kind,
                            };
                        }
                    }
                }
            } else {
                // 是方法函数
            }
        } else {
            // 是 get/set 属性
        }
    }
    return Err("");
}
