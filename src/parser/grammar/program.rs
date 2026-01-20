use crate::{
    lexer::{KeywordToken, Token},
    parser::{
        ProgramParseNode, TokenTraverser, TopLevelDefinition, grammar::{structure, tuple},
    },
};

pub fn program(tokens: &mut TokenTraverser) -> Result<ProgramParseNode, ()> {
    let mut definitions = vec![];
    while !tokens.is_done() {
        definitions.push(match tokens.peek() {
            Token::Keyword(KeywordToken::Tuple) => TopLevelDefinition::Record(tuple(tokens)?),
            Token::Keyword(KeywordToken::Struct) => {
                TopLevelDefinition::Record(structure(tokens)?)
            }
            _ => panic!("Fix this"),
        });
    }
    Ok(ProgramParseNode { definitions })
}
