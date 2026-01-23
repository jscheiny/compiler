use crate::parser::{
    FunctionDefintionParseNode, InterfaceDefinitionParseNode, ParseNode, RecordDefinitionParseNode,
    TokenSpan, Traverse,
};

#[derive(Debug)]
pub struct ProgramParseNode {
    pub definitions: Vec<ParseNode<ModuleTopLevelDefinition>>,
}

impl Traverse for ProgramParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        for definition in self.definitions.iter() {
            definition.traverse(visit);
        }
    }
}

#[derive(Debug)]
pub struct ModuleTopLevelDefinition {
    pub public: bool,
    pub definition: TopLevelDefinition,
}

impl Traverse for ModuleTopLevelDefinition {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        self.definition.traverse(visit);
    }
}

#[derive(Debug)]
pub enum TopLevelDefinition {
    Record(RecordDefinitionParseNode),
    Interface(InterfaceDefinitionParseNode),
    Function(FunctionDefintionParseNode),
}

impl Traverse for TopLevelDefinition {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        match self {
            TopLevelDefinition::Record(node) => node.traverse(visit),
            TopLevelDefinition::Interface(node) => node.traverse(visit),
            TopLevelDefinition::Function(node) => node.traverse(visit),
        }
    }
}
