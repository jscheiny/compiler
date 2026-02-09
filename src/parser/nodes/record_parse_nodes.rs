use crate::parser::{
    IdentifierParseNode, MethodParseNode, ParseNode, ParseNodeVec, RecordMemberParseNode,
    TokenSpan, Traverse,
};

pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: ParseNode<IdentifierParseNode>,
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

pub enum RecordType {
    Struct,
    Tuple,
}
