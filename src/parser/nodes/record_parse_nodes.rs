use crate::parser::{
    IdentifierParseNode, MethodParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse,
    TypeParseNode,
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

pub struct RecordMemberParseNode {
    pub public: bool,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl Traverse for RecordMemberParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("RecordMember.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("RecordMember.type", visit);
        }
    }
}
