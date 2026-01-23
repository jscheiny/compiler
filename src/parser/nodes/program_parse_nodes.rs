use crate::parser::{
    FunctionDefintionParseNode, InterfaceDefinitionParseNode, LocatedNode,
    RecordDefinitionParseNode,
};

#[derive(Debug)]
pub struct ProgramParseNode {
    pub definitions: Vec<LocatedNode<ModuleTopLevelDefinition>>,
}

#[derive(Debug)]
pub struct ModuleTopLevelDefinition {
    pub public: bool,
    pub definition: TopLevelDefinition,
}

#[derive(Debug)]
pub enum TopLevelDefinition {
    Record(RecordDefinitionParseNode),
    Interface(InterfaceDefinitionParseNode),
    Function(FunctionDefintionParseNode),
}
