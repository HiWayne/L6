use crate::tokenizer::Token;

use super::variable_declaration::variable_declaration;

pub fn variable_declaration_list(tokens: &'static [Token], cursor: usize) {
    variable_declaration(tokens, cursor);
}
