use crate::{gram_analysis::statement_list::statement_list, tokenizer::Token};

pub fn program(tokens: &'static [Token]) {
    if tokens.len() > 0 {
        statement_list(tokens, 0);
    }
}
