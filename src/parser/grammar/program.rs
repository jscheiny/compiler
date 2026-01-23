use crate::{
    lexer::{KeywordToken, Token},
    parser::{
        ProgramParseNode, TokenTraverser, TopLevelDefinition,
        grammar::{function, interface, structure, tuple},
    },
};

pub fn program(tokens: &mut TokenTraverser) -> Result<ProgramParseNode, ()> {
    let mut definitions = vec![];
    while !tokens.is_done() {
        if tokens.accept(&KeywordToken::Pub) {
            todo!("Mark as public");
        }

        if let Token::Keyword(keyword) = tokens.peek() {
            use KeywordToken as K;
            let definition = match keyword {
                K::Tuple => TopLevelDefinition::Record(tokens.located(tuple)?),
                K::Struct => TopLevelDefinition::Record(tokens.located(structure)?),
                K::Interface => TopLevelDefinition::Interface(tokens.located(interface)?),
                K::Fn => TopLevelDefinition::Function(function(tokens, true)?),
                _ => panic!("Fix this"),
            };
            definitions.push(definition);
        } else {
            panic!("Not good");
        }
    }
    Ok(ProgramParseNode { definitions })
}
