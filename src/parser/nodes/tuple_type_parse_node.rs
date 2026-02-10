use crate::{
    checker::{TypeResolver, ResolveType, Type},
    parser::{ParseNode, TokenSpan, Traverse, TypeParseNode},
};

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

impl ResolveType for TupleTypeParseNode {
    fn resolve_types(&self, types: &TypeResolver) -> Type {
        let members = self
            .members
            .iter()
            .map(|p| p.value.resolve_types(types))
            .collect();

        Type::Tuple(members)
    }
}
