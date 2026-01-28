use crate::parser::{MethodParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse, TypeParseNode};

#[derive(Debug)]
pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: ParseNode<String>,
    pub members: ParseNodeVec<RecordMemberParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl Traverse for RecordDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Record.identifier", self.identifier.span);
        visit("Record.members", self.members.span);
        for member in self.members.value.iter() {
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
    pub type_def: ParseNode<TypeParseNode>,
}

impl Traverse for RecordMemberParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("RecordMember.identifier", self.identifier.span);
        self.type_def.traverse("RecordMember.type", visit);
    }
}
