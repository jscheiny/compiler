use crate::{
    checker::{ResolveType, Type, TypeResolver},
    parser::{ParseNode, TokenSpan, Traverse, TypeParseNode},
};

pub struct TupleTypeParseNode {
    pub fields: Vec<ParseNode<TypeParseNode>>,
}

impl Traverse for TupleTypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for field in self.fields.iter() {
            field.traverse("TupleType.field", visit);
        }
    }
}

impl ResolveType for TupleTypeParseNode {
    fn resolve_types(&self, types: &TypeResolver) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|p| p.value.resolve_types(types))
            .collect();

        Type::Tuple(fields)
    }
}
