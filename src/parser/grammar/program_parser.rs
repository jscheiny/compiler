use std::rc::Rc;

use crate::{
    lexer::{Keyword, Token},
    parser::{
        ExportableModuleDefinitionNode, ModuleDefinitionNode, ParseResult, ProgramNode,
        SyntaxError, TokenStream,
        grammar::{enumeration, interface, structure, top_level_function, type_alias},
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
    let public = tokens.accept(Keyword::Pub);
    let definition = module_definition(tokens)?;
    Ok(ExportableModuleDefinitionNode { public, definition })
}

fn module_definition(tokens: &mut TokenStream) -> ParseResult<ModuleDefinitionNode> {
    if let Token::Keyword(keyword) = tokens.peek() {
        use Keyword as K;
        use ModuleDefinitionNode as N;
        match keyword {
            K::Enum => Ok(N::Enum(Rc::new(enumeration(tokens)?))),
            K::Fn => Ok(N::Function(Box::new(top_level_function(tokens)?))),
            K::Interface => Ok(N::Interface(Rc::new(interface(tokens)?))),
            K::Struct => Ok(N::Struct(Rc::new(structure(tokens)?))),
            K::Type => Ok(N::TypeAlias(Rc::new(type_alias(tokens)?))),
            _ => Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition)),
        }
    } else {
        Err(tokens.make_error(SyntaxError::ExpectedTopLevelDefinition))
    }
}
