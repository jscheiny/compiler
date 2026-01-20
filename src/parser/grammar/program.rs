use crate::{
    lexer::KeywordToken,
    parser::{
        ProgramParseNode, TokenTraverser, TopLevelDefinition,
        grammar::{function, interface, structure, tuple},
    },
};

pub fn program(tokens: &mut TokenTraverser) -> Result<ProgramParseNode, ()> {
    let mut definitions = vec![];
    while !tokens.is_done() {
        if tokens.accept(&KeywordToken::Tuple) {
            definitions.push(TopLevelDefinition::Record(tuple(tokens)?));
        } else if tokens.accept(&KeywordToken::Struct) {
            definitions.push(TopLevelDefinition::Record(structure(tokens)?));
        } else if tokens.accept(&KeywordToken::Interface) {
            definitions.push(TopLevelDefinition::Interface(interface(tokens)?));
        } else if tokens.accept(&KeywordToken::Fn) {
            definitions.push(TopLevelDefinition::Function(function(tokens)?));
        } else {
            panic!("Fix this!");
        }
    }
    Ok(ProgramParseNode { definitions })
}
