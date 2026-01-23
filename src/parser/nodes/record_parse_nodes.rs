use crate::parser::{
    FunctionDefintionParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse,
    TypeDefinitionParseNode,
};

#[derive(Debug)]
pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: ParseNode<String>,
    pub member_list: ParseNodeVec<RecordMemberParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl Traverse for RecordDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Record.identifier", self.identifier.span);
        visit("Record.members", self.member_list.span);
        for member in self.member_list.value.iter() {
            member.traverse("Record.member", visit);
        }
        if let Some(methods) = self.methods.as_ref() {
            visit("Record.methods", methods.span);
            for method in methods.value.iter() {
                method.traverse("Record.method", visit);
            }
        }
    }
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

impl Traverse for RecordMemberParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("RecordMember.identifier", self.identifier.span);
        self.type_def.traverse("RecordMember.type", visit);
    }
}

#[derive(Debug)]
pub struct MethodParseNode {
    pub public: bool,
    pub function: ParseNode<FunctionDefintionParseNode>,
}

impl Traverse for MethodParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.function.traverse("Method.function", visit);
    }
}
