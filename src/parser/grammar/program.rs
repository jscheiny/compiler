use crate::{
    lexer::KeywordToken,
    parser::{StructDefinitionParseNode, TokenTraverser, grammar::structure},
};

pub fn program(tokens: &mut TokenTraverser) -> Result<StructDefinitionParseNode, ()> {
    if tokens.accept(&KeywordToken::Struct) {
        structure(tokens)
    } else {
        panic!("BAD");
    }
}
