use crate::parser::{
    FunctionDefintionParseNode, LocatedNode, LocatedNodeVec, TypeDefinitionParseNode,
};

#[derive(Debug)]
pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: LocatedNode<String>,
    pub member_list: LocatedNodeVec<RecordMemberParseNode>,
    pub methods: Option<LocatedNodeVec<MethodParseNode>>,
}

#[derive(Debug)]
pub enum RecordType {
    Structure,
    Tuple,
}

#[derive(Debug)]
pub struct RecordMemberParseNode {
    pub public: bool,
    pub identifier: LocatedNode<String>,
    pub type_def: LocatedNode<TypeDefinitionParseNode>,
}

#[derive(Debug)]
pub struct MethodParseNode {
    pub public: bool,
    pub function: LocatedNode<FunctionDefintionParseNode>,
}
