use crate::{
    lexer::{Keyword, Token},
    parser::{
        ExportableModuleDefinitionNode, ModuleDefinitionNode, ParseResult, ProgramNode,
        SyntaxError, TokenStream,
        grammar::{enumeration, structure, top_level_function, type_alias},
    },
};

pub fn program(tokens: &mut TokenStream) -> ParseResult<ProgramNode> {
    let mut definitions = vec![];
    while !tokens.is_done() {
        let definition = tokens.located(exportable_module_definition)?;
        definitions.push(definition);
    }
    Ok(ProgramNode { definitions })
}

fn exportable_module_definition(
    tokens: &mut TokenStream,
) -> ParseResult<ExportableModuleDefinitionNode> {
    let public = tokens.accept(&Keyword::Pub);
    let definition = module_definition(tokens)?;
    Ok(ExportableModuleDefinitionNode { public, definition })
}

fn module_definition(tokens: &mut TokenStream) -> ParseResult<ModuleDefinitionNode> {
    if let Token::Keyword(keyword) = tokens.peek() {
        use Keyword as K;
        match keyword {
            K::Struct => Ok(ModuleDefinitionNode::Struct(structure(tokens)?)),
            K::Enum => Ok(ModuleDefinitionNode::Enum(enumeration(tokens)?)),
            K::Fn => Ok(ModuleDefinitionNode::Function(top_level_function(tokens)?)),
            K::Type => Ok(ModuleDefinitionNode::TypeAlias(type_alias(tokens)?)),
            _ => Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition)),
        }
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition))
    }
}
