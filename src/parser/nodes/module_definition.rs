use crate::parser::{
    EnumParseNode, FunctionParseNode, RecordDefinitionParseNode, TokenSpan, Traverse,
    TypeAliasParseNode,
};

pub enum ModuleDefinition {
    Record(RecordDefinitionParseNode),
    Enum(EnumParseNode),
    Function(FunctionParseNode),
    TypeAlias(TypeAliasParseNode),
}

impl Traverse for ModuleDefinition {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Record(node) => node.traverse(visit),
            Self::Enum(node) => node.traverse(visit),
            Self::Function(node) => node.traverse(visit),
            Self::TypeAlias(node) => node.traverse(visit),
        }
    }
}
