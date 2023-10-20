use crate::{gram_analysis::statement_list::statement_list, tokenizer::Token, types::{Program, ASTType, SourceType}};

pub fn program(tokens: &[Token]) -> Result<Program, &str> {
    let mut ast = Program {
        _type: ASTType::Program,
        body: Vec::new(),
        sourceType: SourceType::module
    };
    if let Ok(mut result) = statement_list(tokens, 0) {
        ast.body.append(&mut result.statements);
        println!("{:#?}", ast);
        return Ok(ast);
    } else {
        return Err("");
    }
}
