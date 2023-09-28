use crate::tokenizer::Token;

use super::statement_list::statement_list;

pub fn block_statement(tokens: &'static [Token], cursor: usize) {
    statement_list(tokens, cursor + 1);
}
