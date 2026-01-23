use crate::parser::{FunctionDefintionParseNode, ParseNode, ParseNodeVec, TypeDefinitionParseNode};

#[derive(Debug)]
pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: ParseNode<String>,
    pub member_list: ParseNodeVec<RecordMemberParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

#[derive(Debug)]
pub enum RecordType {
    Structure,
    Tuple,
}

#[derive(Debug)]
pub struct RecordMemberParseNode {
    pub public: bool,
    pub identifier: ParseNode<String>,
    pub type_def: ParseNode<TypeDefinitionParseNode>,
}

#[derive(Debug)]
pub struct MethodParseNode {
    pub public: bool,
    pub function: ParseNode<FunctionDefintionParseNode>,
}
