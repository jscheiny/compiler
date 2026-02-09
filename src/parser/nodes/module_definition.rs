use crate::parser::{
    EnumParseNode, FunctionParseNode, RecordDefinitionParseNode, TokenSpan, Traverse,
    TypeAliasParseNode,
};

pub struct ExportableModuleDefinitionParseNode {
    pub public: bool,
    pub definition: ModuleDefinitionParseNode,
}

impl Traverse for ExportableModuleDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.definition.traverse(visit);
    }
}

pub enum ModuleDefinitionParseNode {
    Record(RecordDefinitionParseNode),
    Enum(EnumParseNode),
    Function(FunctionParseNode),
    TypeAlias(TypeAliasParseNode),
}

impl Traverse for ModuleDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Record(node) => node.traverse(visit),
            Self::Enum(node) => node.traverse(visit),
            Self::Function(node) => node.traverse(visit),
            Self::TypeAlias(node) => node.traverse(visit),
        }
    }
}
