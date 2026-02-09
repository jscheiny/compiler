use crate::parser::{ParseNode, TokenSpan, Traverse, TypeParseNode};

pub struct TupleTypeParseNode {
    pub members: Vec<ParseNode<TypeParseNode>>,
}

impl Traverse for TupleTypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for member in self.members.iter() {
            member.traverse("TupleType.member", visit);
        }
    }
}
