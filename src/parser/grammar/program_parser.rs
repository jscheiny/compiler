use crate::{
    lexer::{KeywordToken, Token},
    parser::{
        ExportableModuleDefinitionParseNode, ModuleDefinitionParseNode, ParseResult,
        ProgramParseNode, SyntaxError, TokenStream,
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
) -> ParseResult<ExportableModuleDefinitionParseNode> {
    let public = tokens.accept(&KeywordToken::Pub);
    let definition = module_definition(tokens)?;
    Ok(ExportableModuleDefinitionParseNode { public, definition })
}

fn module_definition(tokens: &mut TokenStream) -> ParseResult<ModuleDefinitionParseNode> {
    if let Token::Keyword(keyword) = tokens.peek() {
        use KeywordToken as K;
        match keyword {
            K::Tuple => Ok(ModuleDefinitionParseNode::Record(tuple(tokens)?)),
            K::Struct => Ok(ModuleDefinitionParseNode::Record(structure(tokens)?)),
            K::Enum => Ok(ModuleDefinitionParseNode::Enum(enumeration(tokens)?)),
            K::Fn => Ok(ModuleDefinitionParseNode::Function(top_level_function(
                tokens,
            )?)),
            K::Type => Ok(ModuleDefinitionParseNode::TypeAlias(type_alias(tokens)?)),
            _ => Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition)),
        }
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition))
    }
}
