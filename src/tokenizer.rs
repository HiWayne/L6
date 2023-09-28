// js词法token解析器

use crate::utils::token_trie::{TrieMethods, TrieNode};
use regex::Regex;
use std::{cell::RefCell, fmt::Display, matches};

enum Status {
    Initial,
    Keywords,
    Punctuators,
    Identifier,
    Literal,
    Template,
    Comment,
}

enum LiteralStatus {
    Initial,
    String,
    Int,
    Float,
    RegularExpression,
}

enum TemplateStatus {
    Template,
    Expression,
}

enum CommentStatus {
    SingleLine,
    Multiline,
    None,
}

#[derive(Clone)]
pub struct MyOption<T>(pub Option<T>);

pub struct TokenType {
    pub label: String,
    pub keyword: MyOption<String>,
    pub beforeExpr: bool,
    pub startsExpr: bool,
    pub isLoop: bool,
    pub isAssign: bool,
    pub prefix: bool,
    pub postfix: bool,
}

impl Display for MyOption<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "\"{}\"", value),
            None => write!(f, "undefined"),
        }
    }
}

impl Clone for TokenType {
    fn clone(&self) -> Self {
        TokenType {
            label: self.label.clone(),
            keyword: self.keyword.clone(),
            beforeExpr: self.beforeExpr,
            startsExpr: self.startsExpr,
            isLoop: self.isLoop,
            isAssign: self.isAssign,
            prefix: self.prefix,
            postfix: self.postfix,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n    label: \"{}\",\n    keyword: {},\n    beforeExpr: {},\n    startsExpr: {},\n    isLoop: {},\n    isAssign: {},\n    prefix: {},\n    postfix: {}\n  }}", &self.label, &self.keyword, &self.beforeExpr, &self.startsExpr, &self.isLoop, &self.isAssign, &self.prefix, &self.postfix)
    }
}

#[derive(Clone)]
pub enum TokenValue {
    String(String),
    RegexpValue(RegexpValue),
    None,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenValue::String(value) => write!(f, "\"{}\"", value),
            TokenValue::RegexpValue(value) => write!(f, "{}", value),
            TokenValue::None => write!(f, "undefined"),
        }
    }
}

pub struct Token {
    pub _type: TokenType,
    pub value: TokenValue,
    pub start: u32,
    pub end: u32,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            _type: self._type.clone(),
            value: self.value.clone(),
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n  _type: {},\n  value: {},\n  start: {},\n  end: {}\n}}",
            &self._type, &self.value, &self.start, &self.end
        )
    }
}

#[derive(Clone)]
pub struct RegexpValue {
    pub pattern: String,
    pub flags: String,
    pub value: String,
}

impl Display for RegexpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n      pattern: \"{}\",\n      flags: \"{}\",\n      value: \"{}\"}}",
            &self.pattern, &self.flags, &self.value
        )
    }
}

fn get_literal_status(
    words: &str,
    tokens: &Vec<Token>,
    punctuators_trie_node: &TrieNode,
) -> LiteralStatus {
    // 字符串正则
    let string_regex = Regex::new(r#"^("|'|`)"#).unwrap();
    // 整型正则
    let int_regex = Regex::new(r#"^\d+$"#).unwrap();
    // 浮点数正则
    let float_regex = Regex::new(r#"^\d+(\.\d*)?$"#).unwrap();
    // 正则表达式正则
    let regex_regex = Regex::new(r"^/[^/]?").unwrap();

    if string_regex.is_match(words) {
        return LiteralStatus::String;
    } else if int_regex.is_match(words) {
        return LiteralStatus::Int;
    } else if float_regex.is_match(words) {
        return LiteralStatus::Float;
    } else if punctuators_trie_node.exist(&tokens.last().unwrap()._type.label)
        && regex_regex.is_match(words)
    {
        return LiteralStatus::RegularExpression;
    } else {
        return LiteralStatus::Initial;
    }
}

fn no_escape(number_of_escape_characters: u16) -> bool {
    number_of_escape_characters % 2 == 0
}

struct LexTemplateActionExtractParams<'a> {
    status: &'a mut Status,
    tokens: &'a mut Vec<Token>,
    under_template: &'a mut bool,
    is_start: bool,
}

fn create_lex_template_action_extract_params<'a>(
    status: &'a mut Status,
    tokens: &'a mut Vec<Token>,
    under_template: &'a mut bool,
    is_start: bool,
) -> LexTemplateActionExtractParams<'a> {
    return LexTemplateActionExtractParams {
        status,
        tokens,
        under_template,
        is_start,
    };
}

fn create_lex_template_action(
) -> Box<dyn FnMut(char, u32, u32, &mut LexTemplateActionExtractParams) -> Result<(), String>> {
    let mut template_status = Vec::new();
    let mut template_stack: Vec<char> = Vec::new();
    let mut number_of_escape_characters: u16 = 0;
    let mut cache = String::new();
    let mut token = String::new();

    let closure = move |char: char,
                        start: u32,
                        current_cursor: u32,
                        extract_params: &mut LexTemplateActionExtractParams| {
        if extract_params.is_start {
            template_status.push(TemplateStatus::Template);
        }
        let _current_template_status = template_status.last();
        match _current_template_status {
            Some(current_template_status) => match current_template_status {
                TemplateStatus::Template => {
                    if !cache.is_empty() && char != '{' {
                        token.push_str(&cache);
                        cache = String::new();
                    }
                    if char == '`' && no_escape(number_of_escape_characters) {
                        // 判断是模板字符串开始还是结束
                        // 结束
                        if !extract_params.is_start {
                            extract_params.tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("template"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(String::from(&token)),
                                start,
                                end: current_cursor,
                            });
                            extract_params.tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("`"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::None,
                                start,
                                end: current_cursor,
                            });

                            token = String::new();
                            template_status.pop();
                            template_stack.pop();

                            if template_stack.is_empty() {
                                *extract_params.status = Status::Initial;
                                *extract_params.under_template = false;
                            } else {
                                let current_template_status = template_status.last().unwrap();
                                if let TemplateStatus::Expression = current_template_status {
                                    *extract_params.status = Status::Initial;
                                    *extract_params.under_template = true;
                                }
                            }
                        } else {
                            // 模板字符串开始
                            template_stack.push('`');
                            *extract_params.under_template = true;
                            extract_params.tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("`"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::None,
                                start,
                                end: current_cursor,
                            });
                        }

                        number_of_escape_characters = 0;
                    } else if char == '\\' {
                        number_of_escape_characters += 1;
                        token.push(char);
                    } else if char == '$' && no_escape(number_of_escape_characters) {
                        if cache.is_empty() {
                            cache.push(char);
                        } else if cache.len() == 1 {
                            token.push('$');
                            cache = String::from('$');
                        } else {
                            return Err(String::from("Missing } in template expression"));
                        }
                    } else if char == '{' && no_escape(number_of_escape_characters) {
                        if cache.is_empty() {
                            token.push(char);
                        } else if cache.len() == 1 {
                            extract_params.tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("template"),
                                    keyword: MyOption(None),
                                    beforeExpr: true,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(String::from(&token)),
                                start,
                                end: current_cursor,
                            });
                            extract_params.tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("${"),
                                    keyword: MyOption(None),
                                    beforeExpr: true,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::None,
                                start,
                                end: current_cursor,
                            });
                            template_status.push(TemplateStatus::Expression);
                            token = String::new();
                            cache = String::new();
                            *extract_params.status = Status::Initial;
                        } else {
                            return Err(String::from("Unexpected string"));
                        }
                    } else {
                        number_of_escape_characters = 0;
                        token.push(char);
                    }
                }
                TemplateStatus::Expression => {
                    if char == '}' && no_escape(number_of_escape_characters) {
                        number_of_escape_characters = 0;
                        template_status.pop();
                        extract_params.tokens.push(Token {
                            _type: TokenType {
                                label: String::from("}"),
                                keyword: MyOption(None),
                                beforeExpr: false,
                                startsExpr: false,
                                isLoop: false,
                                isAssign: false,
                                prefix: false,
                                postfix: false,
                            },
                            value: TokenValue::None,
                            start,
                            end: current_cursor,
                        });
                    } else if !extract_params.tokens.is_empty()
                        && extract_params.tokens.last().unwrap()._type.label == "}"
                    {
                        template_status.pop();
                        number_of_escape_characters = 0;
                        if char == '$' {
                            if cache.is_empty() {
                                cache.push(char);
                            } else if cache.len() == 1 {
                                token.push('$');
                                cache = String::from('$');
                            } else {
                                return Err(String::from("Missing } in template expression"));
                            }
                        } else if char == '`' {
                            extract_params.tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("template"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(String::from(&token)),
                                start,
                                end: current_cursor,
                            });
                            token = String::new();

                            template_status.pop();
                            template_stack.pop();

                            if template_stack.is_empty() {
                                *extract_params.status = Status::Initial;
                                *extract_params.under_template = false;
                            } else {
                                let current_template_status = template_status.last().unwrap();
                                if let TemplateStatus::Expression = current_template_status {
                                    *extract_params.status = Status::Initial;
                                    *extract_params.under_template = true;
                                }
                            }

                            extract_params.tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("`"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::None,
                                start,
                                end: current_cursor,
                            });
                        } else if char == '\\' {
                            number_of_escape_characters += 1;
                            token.push(char);
                        } else {
                            token.push(char);
                        }
                    }
                }
            },
            None => {}
        }
        return Ok(());
    };

    return Box::new(closure);
}

struct LexRegularExpressionActionExtractParams<'a> {
    status: &'a mut Status,
    tokens: &'a mut Vec<Token>,
    outer_token: &'a mut String,
    punctuators_trie_node: &'a TrieNode,
    flags_regex: Regex,
    syntax_error_regex: Regex,
    num_regex: Regex,
}

fn create_lex_regular_expression_action_extract_params<'a>(
    status: &'a mut Status,
    tokens: &'a mut Vec<Token>,
    outer_token: &'a mut String,
    punctuators_trie_node: &'a TrieNode,
) -> LexRegularExpressionActionExtractParams<'a> {
    return LexRegularExpressionActionExtractParams {
        status,
        tokens,
        outer_token,
        punctuators_trie_node,
        flags_regex: Regex::new(r"[igmuy]").unwrap(),
        syntax_error_regex: Regex::new(r"([(+?*]|\{\d+\})$").unwrap(),
        num_regex: Regex::new(r"\d").unwrap(),
    };
}

fn create_lex_regular_expression_action() -> Box<
    dyn FnMut(char, u32, u32, &mut LexRegularExpressionActionExtractParams) -> Result<(), String>,
> {
    let mut cache = String::new();
    let mut pattern_start = true;
    let mut flags_start = false;
    // 连续转义符数量
    let mut number_of_escape_characters = 0;
    // 未闭合的圆括号数量
    let mut number_of_brackets = 0;
    // 未闭合的方括号数量
    let mut number_of_square_brackets = 0;
    let mut token = String::new();
    let mut pattern = String::new();
    let mut flags = String::new();

    let mut clear_cache: Box<dyn FnMut(&mut String, &mut String, &mut String)> = Box::new(
        |token: &mut String, pattern: &mut String, cache: &mut String| {
            token.push_str(cache);
            pattern.push_str(cache);
            *cache = String::new();
        },
    );

    let closure =
        move |char: char,
              start: u32,
              current_cursor: u32,
              extract_params: &mut LexRegularExpressionActionExtractParams| {
            if char == '/' && extract_params.outer_token.is_empty() && token.is_empty() {
                token.push(char);
                pattern_start = true;
            } else if char == '/'
                && (!extract_params.outer_token.is_empty() || !token.is_empty())
                && no_escape(number_of_escape_characters)
                && pattern_start
            {
                if !extract_params.outer_token.is_empty() {
                    token.push_str(extract_params.outer_token);
                    pattern.push_str(extract_params.outer_token);
                    *extract_params.outer_token = String::new();
                }
                if number_of_brackets != 0 {
                    let mut error_message = String::from("Invalid regular expression: ");
                    error_message.push_str(&token);
                    error_message.push_str(":");
                    error_message.push_str(" Unterminated group");
                    return Err(error_message);
                }
                if number_of_square_brackets != 0 {
                    return Err(String::from("Unterminated regular expression"));
                }
                token.push(char);
                pattern.push(char);
                number_of_escape_characters = 0;
                pattern_start = false;
                flags_start = true;
            } else if pattern_start {
                if !extract_params.outer_token.is_empty() {
                    token.push_str(extract_params.outer_token);
                    pattern.push_str(extract_params.outer_token);
                    *extract_params.outer_token = String::new();
                }
                if char == '\\' {
                    number_of_escape_characters += 1;
                } else {
                    if char == '{' && no_escape(number_of_escape_characters) {
                        if cache.is_empty() {
                            cache.push('{');
                        } else {
                            clear_cache(&mut token, &mut pattern, &mut cache);
                        }
                    } else if extract_params.num_regex.is_match(&char.to_string()) {
                        if !cache.is_empty() {
                            cache.push(char);
                        }
                    } else if char == '}' && no_escape(number_of_escape_characters) {
                        if cache.len() == 2 {
                            cache.push('}');
                            if extract_params
                                .syntax_error_regex
                                .is_match(&pattern.to_string())
                            {
                                clear_cache(&mut token, &mut pattern, &mut cache);
                                let mut error_message =
                                    String::from("Invalid regular expression: ");
                                error_message.push_str(&token);
                                error_message.push_str(":");
                                error_message.push_str(" Nothing to repeat");
                                return Err(error_message);
                            } else {
                                clear_cache(&mut token, &mut pattern, &mut cache);
                            }
                        } else if !cache.is_empty() {
                            clear_cache(&mut token, &mut pattern, &mut cache);
                        }
                    } else {
                        if !cache.is_empty() {
                            clear_cache(&mut token, &mut pattern, &mut cache);
                        }
                        if char == '(' && no_escape(number_of_escape_characters) {
                            number_of_brackets += 1;
                        } else if char == ')' && no_escape(number_of_escape_characters) {
                            number_of_brackets -= 1;
                        } else if char == '[' && no_escape(number_of_escape_characters) {
                            number_of_square_brackets += 1;
                        } else if char == ']' && no_escape(number_of_escape_characters) {
                            number_of_square_brackets -= 1;
                        } else if (char == '+' || char == '*')
                            && extract_params
                                .syntax_error_regex
                                .is_match(&pattern.to_string())
                        {
                            let mut error_message = String::from("Invalid regular expression: ");
                            error_message.push_str(&token);
                            error_message.push_str(":");
                            error_message.push_str(" Nothing to repeat");
                            return Err(error_message);
                        }
                    }
                    number_of_escape_characters = 0;
                }
                token.push(char);
                pattern.push(char);
            } else if flags_start
                && char != ' '
                && char != '\n'
                && !extract_params
                    .punctuators_trie_node
                    .exist(&char.to_string())
            {
                number_of_escape_characters = 0;
                if extract_params.flags_regex.is_match(&char.to_string()) {
                    token.push(char);
                    flags.push(char);
                } else {
                    return Err(String::from("Invalid regular expression flag"));
                }
            } else {
                flags_start = false;
                number_of_escape_characters = 0;
                extract_params.tokens.push(Token {
                    _type: TokenType {
                        label: String::from("regexp"),
                        keyword: MyOption(None),
                        beforeExpr: false,
                        startsExpr: true,
                        isLoop: false,
                        isAssign: false,
                        prefix: false,
                        postfix: false,
                    },
                    value: TokenValue::RegexpValue(RegexpValue {
                        pattern: String::from(&pattern),
                        flags: String::from(&flags),
                        value: String::from(token.clone()),
                    }),
                    start,
                    end: current_cursor,
                });
                pattern = String::new();
                flags = String::new();
                token = String::new();
                if char == ' ' || char == '\n' {
                    *extract_params.outer_token = String::new();
                    *extract_params.status = Status::Initial;
                } else {
                    *extract_params.status = Status::Punctuators;
                    *extract_params.outer_token = String::from(char);
                }
            }
            return Ok(());
        };
    return Box::new(closure);
}

pub fn tokenizer<'a>(code: &str) -> Result<Vec<Token>, String> {
    // 分词结果
    let tokens_ref_cell: RefCell<Vec<Token>> = RefCell::new(Vec::new());

    let mut ketwords_trie_node = TrieNode::new();

    let mut punctuators_trie_node = TrieNode::new();

    // 关键字
    ketwords_trie_node.insert("var");
    ketwords_trie_node.insert("let");
    ketwords_trie_node.insert("const");
    ketwords_trie_node.insert("function");
    ketwords_trie_node.insert("if");
    ketwords_trie_node.insert("while");
    ketwords_trie_node.insert("for");
    ketwords_trie_node.insert("switch");
    ketwords_trie_node.insert("case");
    ketwords_trie_node.insert("break");
    ketwords_trie_node.insert("continue");
    ketwords_trie_node.insert("async");
    ketwords_trie_node.insert("await");
    ketwords_trie_node.insert("catch");
    ketwords_trie_node.insert("class");
    ketwords_trie_node.insert("debugger");
    ketwords_trie_node.insert("default");
    ketwords_trie_node.insert("delete");
    ketwords_trie_node.insert("do");
    ketwords_trie_node.insert("else");
    ketwords_trie_node.insert("export");
    ketwords_trie_node.insert("extends");
    ketwords_trie_node.insert("finally");
    ketwords_trie_node.insert("import");
    ketwords_trie_node.insert("in");
    ketwords_trie_node.insert("instanceof");
    ketwords_trie_node.insert("new");
    ketwords_trie_node.insert("return");
    ketwords_trie_node.insert("super");
    ketwords_trie_node.insert("this");
    ketwords_trie_node.insert("throw");
    ketwords_trie_node.insert("try");
    ketwords_trie_node.insert("typeof");
    ketwords_trie_node.insert("void");
    ketwords_trie_node.insert("with");
    ketwords_trie_node.insert("yield");
    ketwords_trie_node.insert("enum");
    ketwords_trie_node.insert("true");
    ketwords_trie_node.insert("false");
    ketwords_trie_node.insert("null");

    // punctuators 符号
    punctuators_trie_node.insert("=");
    punctuators_trie_node.insert(";");
    punctuators_trie_node.insert("(");
    punctuators_trie_node.insert(")");
    punctuators_trie_node.insert("{");
    punctuators_trie_node.insert("}");
    punctuators_trie_node.insert("+");
    punctuators_trie_node.insert("-");
    punctuators_trie_node.insert("*");
    punctuators_trie_node.insert("%");
    punctuators_trie_node.insert("\\");
    punctuators_trie_node.insert(".");
    punctuators_trie_node.insert("!");
    punctuators_trie_node.insert(",");
    punctuators_trie_node.insert("==");
    punctuators_trie_node.insert("===");
    punctuators_trie_node.insert("!=");
    punctuators_trie_node.insert("!==");
    punctuators_trie_node.insert(">");
    punctuators_trie_node.insert("<");
    punctuators_trie_node.insert(">=");
    punctuators_trie_node.insert("<=");
    punctuators_trie_node.insert("=>");
    punctuators_trie_node.insert("+=");
    punctuators_trie_node.insert("-=");
    punctuators_trie_node.insert("*=");
    punctuators_trie_node.insert("/=");
    punctuators_trie_node.insert("%=");
    punctuators_trie_node.insert("...");
    punctuators_trie_node.insert("&&");
    punctuators_trie_node.insert("||");
    punctuators_trie_node.insert("**");
    punctuators_trie_node.insert("++");
    punctuators_trie_node.insert("--");
    punctuators_trie_node.insert("<<");
    punctuators_trie_node.insert(">>");
    punctuators_trie_node.insert(">>>");
    punctuators_trie_node.insert("&");
    punctuators_trie_node.insert("|");
    punctuators_trie_node.insert("^");
    punctuators_trie_node.insert("~");
    punctuators_trie_node.insert("?");
    punctuators_trie_node.insert(":");
    punctuators_trie_node.insert("**=");
    punctuators_trie_node.insert("<<=");
    punctuators_trie_node.insert(">>=");
    punctuators_trie_node.insert(">>>=");
    punctuators_trie_node.insert("&=");
    punctuators_trie_node.insert("|=");
    punctuators_trie_node.insert("^=");

    let init_string_mode = '\0';

    let mut current_cursor: u32 = 0;
    let mut current_column: u32 = 0;
    let mut start: u32 = 0;
    let mut start_column: u32 = 0;
    let mut start: u32 = 0;
    let mut end: u32 = 0;
    let mut current_cusrosr: u32 = 0;
    let status_ref_cell = RefCell::new(Status::Initial);
    // 0-初始化、1-字符串、2-整型、3-浮点数、4-boolean、5-null、6-undefined
    let mut literal_status: LiteralStatus = LiteralStatus::Initial;
    // 字符串的模式：双引号、单引号、模板字符串
    let mut string_mode = init_string_mode;
    let under_template_ref_cell = RefCell::new(false);
    // 字符串中连续转义符的数量
    let mut number_of_escape_characters: u16 = 0;
    let token_ref_cell = RefCell::new(String::new());
    let mut current_keywords_node = &ketwords_trie_node;
    let mut current_punctuators_node = &punctuators_trie_node;

    let mut tokens = tokens_ref_cell.borrow_mut();

    let mut status = status_ref_cell.borrow_mut();

    let mut under_template = under_template_ref_cell.borrow_mut();

    let mut comment_status = CommentStatus::None;

    let mut token = token_ref_cell.borrow_mut();

    let mut lex_regular_expression_action = create_lex_regular_expression_action();

    let mut lex_template_action = create_lex_template_action();

    for char in code.chars() {
        // 更新token的开始位置
        if token.is_empty() {
            start = current_cusrosr;
        }
        current_cusrosr += 1;
        match *status {
            // 初始状态
            Status::Initial => {
                if let Some(next_keywords_node) = current_keywords_node.reduce_find(&char) {
                    *status = Status::Keywords;
                    token.push(char);
                    current_keywords_node = next_keywords_node;
                } else {
                    // 模板字符串
                    if char == '`' {
                        *status = Status::Template;
                        let mut lex_template_action_extract_params =
                            create_lex_template_action_extract_params(
                                &mut status,
                                &mut tokens,
                                &mut under_template,
                                true,
                            );
                        if let Err(error_message) = lex_template_action(
                            char,
                            start,
                            current_cursor,
                            &mut lex_template_action_extract_params,
                        ) {
                            return Err(error_message);
                        }
                    // 遇到符号
                    } else if let Some(next_punctuators_node) =
                        current_punctuators_node.reduce_find(&char)
                    {
                        *status = Status::Punctuators;
                        token.push(char);
                        current_punctuators_node = next_punctuators_node;
                    // 遇到空格或换行结束
                    } else if char != ' ' && char != '\n' {
                        token.push(char);
                        literal_status =
                            get_literal_status(&char.to_string(), &tokens, &punctuators_trie_node);
                        // 可能是字面量
                        if !matches!(literal_status, LiteralStatus::Initial) {
                            *status = Status::Literal;
                            // 如果是字符串需要确定字符串模式
                            if char == '"' || char == '\'' {
                                string_mode = char;
                            }
                        // 标识符
                        } else {
                            *status = Status::Identifier;
                        }
                    }
                }
            }
            // 关键字状态
            Status::Keywords => {
                if let Some(next_keywords_node) = current_keywords_node.reduce_find(&char) {
                    token.push(char);
                    current_keywords_node = next_keywords_node;
                } else {
                    // 已经收集到了完整的关键字
                    if current_keywords_node.is_end_of_word {
                        // 有符号则代表结束
                        if let Some(next_punctuators_node) =
                            current_punctuators_node.reduce_find(&char)
                        {
                            *status = Status::Punctuators;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from(token.clone()),
                                    keyword: MyOption(Some(String::from(token.clone()))),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::from(char);
                            current_punctuators_node = next_punctuators_node

                        // 有空格或换行则代表结束
                        } else if char == ' ' || char == '\n' {
                            *status = Status::Initial;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from(token.clone()),
                                    keyword: MyOption(Some(String::from(token.clone()))),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::new();
                        } else if char == '`' {
                            // 模板字符串
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from(token.clone()),
                                    keyword: MyOption(Some(String::from(token.clone()))),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::new();
                            *status = Status::Template;
                            let mut lex_template_action_extract_params =
                                create_lex_template_action_extract_params(
                                    &mut status,
                                    &mut tokens,
                                    &mut under_template,
                                    true,
                                );
                            if let Err(error_message) = lex_template_action(
                                char,
                                start,
                                current_cursor,
                                &mut lex_template_action_extract_params,
                            ) {
                                return Err(error_message);
                            }
                        } else {
                            // 否则是其他情况（前缀和关键字一样）
                            literal_status =
                                get_literal_status(&token, &tokens, &punctuators_trie_node);
                            // 紧接字面量
                            if !matches!(literal_status, LiteralStatus::Initial) {
                                *status = Status::Literal;
                                tokens.push(Token {
                                    _type: TokenType {
                                        label: String::from(token.clone()),
                                        keyword: MyOption(Some(String::from(token.clone()))),
                                        beforeExpr: false,
                                        startsExpr: false,
                                        isLoop: false,
                                        isAssign: false,
                                        prefix: false,
                                        postfix: false,
                                    },
                                    value: TokenValue::String(token.clone()),
                                    start,
                                    end: current_cursor,
                                });
                                *token = String::from(char);
                                if char == '"' || char == '\'' {
                                    string_mode = char;
                                }
                            } else {
                                // 标识符
                                *status = Status::Identifier;
                                token.push(char);
                            }
                        }
                    // 不是关键字，只是前缀和关键字重合
                    } else {
                        // 遇到符号结束
                        if let Some(next_punctuators_node) =
                            current_punctuators_node.reduce_find(&char)
                        {
                            *status = Status::Punctuators;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("name"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::from(char);
                            current_punctuators_node = next_punctuators_node;
                        // token有值时遇到空格或换行结束
                        } else if char == ' ' || char == '\n' {
                            *status = Status::Initial;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("name"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::new();
                        } else if char == '`' {
                            // 模板字符串
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("name"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: true,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::new();
                            *status = Status::Template;
                            let mut lex_template_action_extract_params =
                                create_lex_template_action_extract_params(
                                    &mut status,
                                    &mut tokens,
                                    &mut under_template,
                                    true,
                                );
                            if let Err(error_message) = lex_template_action(
                                char,
                                start,
                                current_cursor,
                                &mut lex_template_action_extract_params,
                            ) {
                                return Err(error_message);
                            }
                        } else {
                            // 紧接字符串字面量
                            if char == '"' || char == '\'' {
                                *status = Status::Literal;
                                literal_status = LiteralStatus::String;
                                tokens.push(Token {
                                    _type: TokenType {
                                        label: String::from("name"),
                                        keyword: MyOption(None),
                                        beforeExpr: false,
                                        startsExpr: true,
                                        isLoop: false,
                                        isAssign: false,
                                        prefix: false,
                                        postfix: false,
                                    },
                                    value: TokenValue::String(token.clone()),
                                    start,
                                    end: current_cursor,
                                });
                                *token = String::from(char);
                                if char == '"' || char == '\'' {
                                    string_mode = char;
                                }
                            } else {
                                // 标识符
                                *status = Status::Identifier;
                                token.push(char);
                            }
                        }
                    }
                    current_keywords_node = &ketwords_trie_node;
                }
            }
            // 符号状态
            Status::Punctuators => {
                if std::ptr::eq(current_punctuators_node, &punctuators_trie_node) {
                    if let Some(char) = token.chars().next() {
                        if let Some(next_punctuators_node) =
                            current_punctuators_node.reduce_find(&char)
                        {
                            current_punctuators_node = next_punctuators_node;
                        }
                    }
                }
                if let Some(next_punctuators_node) = current_punctuators_node.reduce_find(&char) {
                    token.push(char);
                    current_punctuators_node = next_punctuators_node;
                } else {
                    if *token == "}" && *under_template {
                        current_punctuators_node = &punctuators_trie_node;
                        *status = Status::Template;
                        tokens.push(Token {
                            _type: TokenType {
                                label: String::from("}"),
                                keyword: MyOption(None),
                                beforeExpr: false,
                                startsExpr: false,
                                isLoop: false,
                                isAssign: false,
                                prefix: false,
                                postfix: false,
                            },
                            value: TokenValue::None,
                            start,
                            end: current_cursor,
                        });
                        *token = String::new();
                        let mut lex_template_action_extract_params =
                            create_lex_template_action_extract_params(
                                &mut status,
                                &mut tokens,
                                &mut under_template,
                                false,
                            );
                        if let Err(error_message) = lex_template_action(
                            char,
                            start,
                            current_cursor,
                            &mut lex_template_action_extract_params,
                        ) {
                            return Err(error_message);
                        }
                    } else if current_punctuators_node.is_end_of_word {
                        current_punctuators_node = &punctuators_trie_node;
                        tokens.push(Token {
                            _type: TokenType {
                                label: token.to_string(),
                                keyword: MyOption(None),
                                beforeExpr: true,
                                startsExpr: false,
                                isLoop: false,
                                isAssign: false,
                                prefix: false,
                                postfix: false,
                            },
                            value: TokenValue::String(token.to_string()),
                            start,
                            end: current_cursor,
                        });
                        // 以一个新的符号开始
                        if let Some(next_punctuators_node) =
                            current_punctuators_node.reduce_find(&char)
                        {
                            current_punctuators_node = next_punctuators_node;
                            *token = String::from(char);
                        // 后面是关键字
                        } else if let Some(next_keywords_node) =
                            current_keywords_node.reduce_find(&char)
                        {
                            *status = Status::Keywords;
                            current_keywords_node = next_keywords_node;
                            *token = String::from(char);
                        } else if char == ' ' || char == '\n' {
                            *status = Status::Initial;
                            *token = String::new();
                        } else if char == '`' {
                            // 模板字符串
                            tokens.push(Token {
                                _type: TokenType {
                                    label: token.to_string(),
                                    keyword: MyOption(None),
                                    beforeExpr: true,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.to_string()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::new();
                            *status = Status::Template;
                            let mut lex_template_action_extract_params =
                                create_lex_template_action_extract_params(
                                    &mut status,
                                    &mut tokens,
                                    &mut under_template,
                                    true,
                                );
                            if let Err(error_message) = lex_template_action(
                                char,
                                start,
                                current_cursor,
                                &mut lex_template_action_extract_params,
                            ) {
                                return Err(error_message);
                            }
                        } else {
                            *token = String::from(char);
                            literal_status = get_literal_status(
                                &char.to_string(),
                                &tokens,
                                &punctuators_trie_node,
                            );
                            // 后面是字面量
                            if !matches!(literal_status, LiteralStatus::Initial) {
                                *status = Status::Literal;
                                if char == '"' || char == '\'' {
                                    string_mode = char;
                                }
                            } else {
                                // 后面是标识符
                                *status = Status::Identifier;
                            }
                        }
                    } else if *token == "/" {
                        current_punctuators_node = &punctuators_trie_node;
                        if char == '/' {
                            // 单行注释
                            *status = Status::Comment;
                            comment_status = CommentStatus::SingleLine;
                        } else if char == '*' {
                            // 多行注释
                            *status = Status::Comment;
                            comment_status = CommentStatus::Multiline;
                        } else if punctuators_trie_node.exist(&tokens.last().unwrap()._type.label) {
                            // 正则
                            *status = Status::Literal;
                            token.push(char);
                            literal_status = LiteralStatus::RegularExpression;
                        }
                    } else {
                        return Err(String::from("unknown error"));
                    }
                }
            }
            // 标识符状态
            Status::Identifier => {
                // 遇到符号结束
                if let Some(next_punctuators_node) = current_punctuators_node.reduce_find(&char) {
                    *status = Status::Punctuators;
                    tokens.push(Token {
                        _type: TokenType {
                            label: String::from("name"),
                            keyword: MyOption(None),
                            beforeExpr: false,
                            startsExpr: true,
                            isLoop: false,
                            isAssign: false,
                            prefix: false,
                            postfix: false,
                        },
                        value: TokenValue::String(token.clone()),
                        start,
                        end: current_cursor,
                    });
                    *token = String::from(char);
                    current_punctuators_node = next_punctuators_node;

                // 遇到空格或换行结束
                } else if char == ' ' || char == '\n' {
                    *status = Status::Initial;
                    tokens.push(Token {
                        _type: TokenType {
                            label: String::from("name"),
                            keyword: MyOption(None),
                            beforeExpr: false,
                            startsExpr: true,
                            isLoop: false,
                            isAssign: false,
                            prefix: false,
                            postfix: false,
                        },
                        value: TokenValue::String(token.clone()),
                        start,
                        end: current_cursor,
                    });
                    *token = String::new();
                } else if char == '`' {
                    // 模板字符串
                    tokens.push(Token {
                        _type: TokenType {
                            label: String::from("name"),
                            keyword: MyOption(None),
                            beforeExpr: false,
                            startsExpr: true,
                            isLoop: false,
                            isAssign: false,
                            prefix: false,
                            postfix: false,
                        },
                        value: TokenValue::String(token.clone()),
                        start,
                        end: current_cursor,
                    });
                    *token = String::new();
                    *status = Status::Template;
                    let mut lex_template_action_extract_params =
                        create_lex_template_action_extract_params(
                            &mut status,
                            &mut tokens,
                            &mut under_template,
                            true,
                        );
                    if let Err(error_message) = lex_template_action(
                        char,
                        start,
                        current_cursor,
                        &mut lex_template_action_extract_params,
                    ) {
                        return Err(error_message);
                    }
                } else {
                    literal_status =
                        get_literal_status(&char.to_string(), &tokens, &punctuators_trie_node);
                    if !matches!(literal_status, LiteralStatus::Initial) {
                        *status = Status::Literal;
                        tokens.push(Token {
                            _type: TokenType {
                                label: String::from("name"),
                                keyword: MyOption(None),
                                beforeExpr: false,
                                startsExpr: true,
                                isLoop: false,
                                isAssign: false,
                                prefix: false,
                                postfix: false,
                            },
                            value: TokenValue::String(token.clone()),
                            start,
                            end: current_cursor,
                        });
                        *token = String::from(char);
                        if char == '"' || char == '\'' {
                            string_mode = char;
                        }
                    } else {
                        token.push(char);
                    }
                }
            }
            // 字面量状态
            Status::Literal => {
                match literal_status {
                    // 字符串字面量
                    LiteralStatus::String => {
                        // 普通字符串
                        if string_mode == '"' || string_mode == '\'' {
                            token.push(char);
                            if char == '\\' {
                                number_of_escape_characters += 1
                            } else {
                                // 字符串结束
                                if number_of_escape_characters % 2 == 0 && char == string_mode {
                                    *status = Status::Initial;
                                    literal_status = LiteralStatus::Initial;
                                    tokens.push(Token {
                                        _type: TokenType {
                                            label: String::from("string"),
                                            keyword: MyOption(None),
                                            beforeExpr: false,
                                            startsExpr: false,
                                            isLoop: false,
                                            isAssign: false,
                                            prefix: false,
                                            postfix: false,
                                        },
                                        value: TokenValue::String(token.clone()),
                                        start,
                                        end: current_cursor,
                                    });
                                    *token = String::new();
                                }
                                number_of_escape_characters = 0
                            }
                        }
                    }
                    // 整型字面量
                    LiteralStatus::Int => {
                        let num_regexp = Regex::new(r"\d").unwrap();
                        if num_regexp.is_match(&char.to_string()) {
                            token.push(char);
                        } else if char == '.' {
                            token.push(char);
                            literal_status = LiteralStatus::Float;
                        } else if let Some(next_punctuators_node) =
                            current_punctuators_node.reduce_find(&char)
                        {
                            *status = Status::Punctuators;
                            literal_status = LiteralStatus::Initial;
                            current_punctuators_node = next_punctuators_node;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("number"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::from(char);
                        } else if let Some(next_keywords_node) =
                            current_keywords_node.reduce_find(&char)
                        {
                            *status = Status::Keywords;
                            literal_status = LiteralStatus::Initial;
                            current_keywords_node = next_keywords_node;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("number"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::from(char);
                        } else if char == ' ' || char == '\n' {
                            *status = Status::Initial;
                            literal_status = LiteralStatus::Initial;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("number"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::new();
                        } else {
                            literal_status = get_literal_status(
                                &char.to_string(),
                                &tokens,
                                &punctuators_trie_node,
                            );
                            if !matches!(literal_status, LiteralStatus::Initial) {
                                tokens.push(Token {
                                    _type: TokenType {
                                        label: String::from("number"),
                                        keyword: MyOption(None),
                                        beforeExpr: false,
                                        startsExpr: false,
                                        isLoop: false,
                                        isAssign: false,
                                        prefix: false,
                                        postfix: false,
                                    },
                                    value: TokenValue::String(token.clone()),
                                    start,
                                    end: current_cursor,
                                });
                                *token = String::from(char);
                            } else {
                                return Err(String::from("Identifier directly after number"));
                            }
                        }
                    }
                    // 浮点数字面量
                    LiteralStatus::Float => {
                        let num_regexp = Regex::new(r"\d").unwrap();
                        if num_regexp.is_match(&char.to_string()) {
                            token.push(char);
                        } else if char == '.' {
                            return Err(String::from("Unexpected number"));
                        } else if let Some(next_punctuators_node) =
                            current_punctuators_node.reduce_find(&char)
                        {
                            *status = Status::Punctuators;
                            literal_status = LiteralStatus::Initial;
                            current_punctuators_node = next_punctuators_node;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("number"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::from(char);
                        } else if let Some(next_keywords_node) =
                            current_keywords_node.reduce_find(&char)
                        {
                            *status = Status::Keywords;
                            literal_status = LiteralStatus::Initial;
                            current_keywords_node = next_keywords_node;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("number"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::from(char);
                        } else if char == ' ' || char == '\n' {
                            *status = Status::Initial;
                            literal_status = LiteralStatus::Initial;
                            tokens.push(Token {
                                _type: TokenType {
                                    label: String::from("number"),
                                    keyword: MyOption(None),
                                    beforeExpr: false,
                                    startsExpr: false,
                                    isLoop: false,
                                    isAssign: false,
                                    prefix: false,
                                    postfix: false,
                                },
                                value: TokenValue::String(token.clone()),
                                start,
                                end: current_cursor,
                            });
                            *token = String::new();
                        } else {
                            literal_status = get_literal_status(
                                &char.to_string(),
                                &tokens,
                                &punctuators_trie_node,
                            );
                            if !matches!(literal_status, LiteralStatus::Initial) {
                                tokens.push(Token {
                                    _type: TokenType {
                                        label: String::from("number"),
                                        keyword: MyOption(None),
                                        beforeExpr: false,
                                        startsExpr: false,
                                        isLoop: false,
                                        isAssign: false,
                                        prefix: false,
                                        postfix: false,
                                    },
                                    value: TokenValue::String(token.clone()),
                                    start,
                                    end: current_cursor,
                                });
                                *token = String::from(char);
                            } else {
                                return Err(String::from("Identifier directly after number"));
                            }
                        }
                    }
                    // 正则字面量
                    LiteralStatus::RegularExpression => {
                        let mut lex_regular_expression_action_extract_params =
                            create_lex_regular_expression_action_extract_params(
                                &mut status,
                                &mut tokens,
                                &mut token,
                                &punctuators_trie_node,
                            );
                        if let Err(error_message) = lex_regular_expression_action(
                            char,
                            start,
                            current_cursor,
                            &mut lex_regular_expression_action_extract_params,
                        ) {
                            return Err(error_message);
                        }
                    }
                    LiteralStatus::Initial => return Err(String::from("unknown error")),
                }
            }
            // 模板字符串状态
            Status::Template => {
                let mut lex_template_action_extract_params =
                    create_lex_template_action_extract_params(
                        &mut status,
                        &mut tokens,
                        &mut under_template,
                        false,
                    );
                if let Err(error_message) = lex_template_action(
                    char,
                    start,
                    current_cursor,
                    &mut lex_template_action_extract_params,
                ) {
                    return Err(error_message);
                }
            }
            Status::Comment => match comment_status {
                CommentStatus::SingleLine => {
                    if char == '\n' {
                        *token = String::new();
                        *status = Status::Initial;
                        comment_status = CommentStatus::None;
                    }
                }
                CommentStatus::Multiline => {
                    let len = token.len();
                    let last_char = token.chars().nth(len - 1).unwrap();
                    if last_char == '*' && char == '/' {
                        *token = String::new();
                        *status = Status::Initial;
                        comment_status = CommentStatus::None;
                    } else {
                        token.push(char);
                    }
                }
                CommentStatus::None => return Err(String::from("unknown error")),
            },
        }
    }
    // TODO:非初始状态下未匹配完就结束的情况
    return Ok(tokens.clone());
}
