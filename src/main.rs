mod tokenizer;

mod types;

mod utils {
    pub mod token_trie;
}

mod gram_analysis {
    pub mod array_expression;
    pub mod assignment_expression;
    pub mod block_statement;
    pub mod conditional_expression;
    pub mod element_list;
    pub mod expression;
    pub mod identifier;
    pub mod literal_expression;
    pub mod logical_and_expression;
    pub mod logical_or_expression;
    pub mod object_expression;
    pub mod object_property;
    pub mod program;
    pub mod statement;
    pub mod statement_list;
    pub mod variable_declaration;
    pub mod variable_declaration_list;
    pub mod variable_statement;
    pub mod function_expression;
}

use crate::gram_analysis::program::program;
use crate::tokenizer::{tokenizer, TokenValue};

/**
* <Program> ::= <StatementList>
* <StatementList> ::= <Statement> | <Statement> <StatementList>
* <Statement> ::= <BlockStatement>
                | <VariableStatement>
                | <EmptyStatement>
                | <ExpressionStatement>
                | <IfStatement>
                | <IterationStatement>
                | <ContinueStatement>
                | <BreakStatement>
                | <ReturnStatement>
                | <WithStatement>
                | <LabelledStatement>
                | <SwitchStatement>
                | <ThrowStatement>
                | <TryStatement>
                | <DebuggerStatement>
* <BlockStatement> ::= '{' <StatementList>? '}'
* <VariableStatement> ::= 'var' <VariableDeclarationList> ';' | 'let' <VariableDeclarationList> ';' | 'const' <VariableDeclaration> ';'
* <VariableDeclarationList> ::= <VariableDeclaration> | <VariableDeclaration> ',' <VariableDeclarationList>
* <VariableDeclaration> ::= <Identifier> '=' <Expression>
* <OperatorExpression> ::= <Expression> <Operator> <Expression>
* <ConditionalExpression> ::= <Expression> '?' <Expression> ':' <Expression>
* <LogicalOrExpression> ::= ('!')+ <Expression>
* <Expression> ::= <LiteralExpression> | <ArrayExpression> | <ObjectExpression> | <NewExpression> | <CallExpression> | <ConditionalExpression> | <AssignmentExpression> | <LogicalOrExpression> | <FunctionExpression> | <ArrowFunctionExpression>
* <Operator> ::= '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '==' | '===' | '!=' | '!==' | '+' | '-' | '*' | '/' | '%' | '>' | '>=' | '<' | '<=' | '&&' | '||' | '^' | '~' | '|' | '&' | '<<' | '>>' | '>>>'
* <Identifier> ::= /^[a-zA-Z_$\u00A0-\uFFFF][a-zA-Z0-9_$\u00A0-\uFFFF]*$/
* <ArrayExpression> ::= '[' <ElementList>? ']'
* <ElementList> ::= (<Identifier> | <Expression>) (',' <ElementList>)*
* <ObjectExpression> ::= '{' <ObjectProperty>? '}'
* <ObjectProperty> ::= ('[' <Identifier> ']' ':' | <Identifier> | <string>) <Expression> (',' <ObjectProperty>)*
*/

fn main() {
    let code = "const t1 = [1, 2];{let t2 = \"abc\"}";

    match tokenizer(code) {
        Ok(tokens) => {
            for token in tokens.iter() {
                // match &token.value {
                //     TokenValue::String(string) => {
                //         println!("tokens: {}", string)
                //     }
                //     TokenValue::RegexpValue(regexp_value) => {
                //         println!("tokens: {}", regexp_value.value);
                //     }
                //     TokenValue::None => {
                //         println!("tokens: {}", token._type.label);
                //     }
                // }
                println!("{}", &token)
            }
            program(&tokens);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
