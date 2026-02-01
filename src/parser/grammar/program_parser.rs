use crate::{
    lexer::{KeywordToken, Token},
    parser::{
        ModuleTopLevelDefinition, ParseResult, ProgramParseNode, SyntaxError, TokenStream,
        TopLevelDefinition,
        grammar::{enumeration, interface, structure, top_level_function, tuple},
    },
};

pub fn program(tokens: &mut TokenStream) -> ParseResult<ProgramParseNode> {
    let mut definitions = vec![];
    while !tokens.is_done() {
        let definition = tokens.located(module_top_level_definition)?;
        definitions.push(definition);
    }
    Ok(ProgramParseNode { definitions })
}

fn module_top_level_definition(tokens: &mut TokenStream) -> ParseResult<ModuleTopLevelDefinition> {
    let public = tokens.accept(&KeywordToken::Pub);
    let definition = top_level_definition(tokens)?;
    Ok(ModuleTopLevelDefinition { public, definition })
}

fn top_level_definition(tokens: &mut TokenStream) -> ParseResult<TopLevelDefinition> {
    if let Token::Keyword(keyword) = tokens.peek() {
        use KeywordToken as K;
        match keyword {
            K::Tuple => Ok(TopLevelDefinition::Record(tuple(tokens)?)),
            K::Struct => Ok(TopLevelDefinition::Record(structure(tokens)?)),
            K::Enum => Ok(TopLevelDefinition::Enum(enumeration(tokens)?)),
            K::Interface => Ok(TopLevelDefinition::Interface(interface(tokens)?)),
            K::Fn => Ok(TopLevelDefinition::Function(top_level_function(tokens)?)),
            _ => Err(tokens.make_error(SyntaxError::Unimplemented)),
        }
    } else {
        Err(tokens.make_error(SyntaxError::Unimplemented))
    }
}
