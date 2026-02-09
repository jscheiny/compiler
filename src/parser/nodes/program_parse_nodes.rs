use crate::parser::{
    EnumParseNode, FunctionParseNode, InterfaceDefinitionParseNode, ParseNode,
    RecordDefinitionParseNode, TokenSpan, Traverse, TypeAliasParseNode,
};

pub struct ProgramParseNode {
    pub definitions: Vec<ParseNode<ModuleTopLevelDefinition>>,
}

impl Traverse for ProgramParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for definition in self.definitions.iter() {
            definition.traverse("Program.definition", visit);
        }
    }
}

pub struct ModuleTopLevelDefinition {
    pub public: bool,
    pub definition: TopLevelDefinition,
}

impl Traverse for ModuleTopLevelDefinition {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.definition.traverse(visit);
    }
}

pub enum TopLevelDefinition {
    Record(RecordDefinitionParseNode),
    Enum(EnumParseNode),
    Interface(InterfaceDefinitionParseNode),
    Function(FunctionParseNode),
    TypeAlias(TypeAliasParseNode),
}

impl Traverse for TopLevelDefinition {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Record(node) => node.traverse(visit),
            Self::Enum(node) => node.traverse(visit),
            Self::Interface(node) => node.traverse(visit),
            Self::Function(node) => node.traverse(visit),
            Self::TypeAlias(node) => node.traverse(visit),
        }
    }
}
