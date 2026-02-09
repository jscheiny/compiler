use crate::{
    lexer::{KeywordToken, Token},
    parser::{
        ExportableModuleDefinition, ModuleDefinition, ParseResult, ProgramParseNode, SyntaxError,
        TokenStream,
        grammar::{enumeration, structure, top_level_function, tuple, type_alias},
    },
};

pub fn program(tokens: &mut TokenStream) -> ParseResult<ProgramParseNode> {
    let mut definitions = vec![];
    while !tokens.is_done() {
        let definition = tokens.located(exportable_module_definition)?;
        definitions.push(definition);
    }
    Ok(ProgramParseNode { definitions })
}

fn exportable_module_definition(
    tokens: &mut TokenStream,
) -> ParseResult<ExportableModuleDefinition> {
    let public = tokens.accept(&KeywordToken::Pub);
    let definition = module_definition(tokens)?;
    Ok(ExportableModuleDefinition { public, definition })
}

fn module_definition(tokens: &mut TokenStream) -> ParseResult<ModuleDefinition> {
    if let Token::Keyword(keyword) = tokens.peek() {
        use KeywordToken as K;
        match keyword {
            K::Tuple => Ok(ModuleDefinition::Record(tuple(tokens)?)),
            K::Struct => Ok(ModuleDefinition::Record(structure(tokens)?)),
            K::Enum => Ok(ModuleDefinition::Enum(enumeration(tokens)?)),
            K::Fn => Ok(ModuleDefinition::Function(top_level_function(tokens)?)),
            K::Type => Ok(ModuleDefinition::TypeAlias(type_alias(tokens)?)),
            _ => Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition)),
        }
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition))
    }
}
