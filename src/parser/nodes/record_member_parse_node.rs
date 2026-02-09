use crate::parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode};

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
