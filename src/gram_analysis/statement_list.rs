use crate::tokenizer::Token;
use crate::gram_analysis::statement::statement;

pub fn statement_list(tokens: &'static [Token], cursor: usize) {
    statement(tokens, cursor);
}
