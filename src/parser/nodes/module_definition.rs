use crate::{
    checker::TypeResolver,
    parser::{EnumParseNode, FunctionParseNode, Identified, StructParseNode, TypeAliasParseNode},
};

pub struct ExportableModuleDefinitionParseNode {
    pub public: bool,
    pub definition: ModuleDefinitionParseNode,
}

pub enum ModuleDefinitionParseNode {
    Struct(StructParseNode),
    Enum(EnumParseNode),
    Function(FunctionParseNode),
    TypeAlias(TypeAliasParseNode),
}

impl ModuleDefinitionParseNode {
    pub fn identifier(&self) -> &String {
        match self {
            Self::Struct(node) => node.id(),
            Self::Enum(node) => node.id(),
            Self::TypeAlias(node) => node.id(),
            Self::Function(node) => &node.identifier.value.0,
        }
    }

    pub fn resolve_type(&mut self, types: &mut TypeResolver) {
        match self {
            Self::Struct(node) => node.resolve_types(types),
            Self::Enum(node) => node.resolve_types(types),
            Self::TypeAlias(node) => node.resolve_types(types),
            Self::Function(_) => {}
        }
    }
}
