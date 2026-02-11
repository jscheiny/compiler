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
    pub fn resolve_type(&mut self, types: &mut TypeResolver) {
        match self {
            Self::Struct(node) => node.resolve_type(types),
            Self::Enum(node) => node.resolve_type(types),
            Self::TypeAlias(node) => types.resolve(node.id(), node.get_type(types).clone()),
            Self::Function(_) => {}
        }
    }
}

impl Identified for ModuleDefinitionParseNode {
    fn id(&self) -> &String {
        match self {
            Self::Struct(node) => node.id(),
            Self::Enum(node) => node.id(),
            Self::TypeAlias(node) => node.id(),
            Self::Function(node) => node.id(),
        }
    }
}
