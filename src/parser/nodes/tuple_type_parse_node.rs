use crate::{
    checker::{Type, TypeResolver},
    parser::{ParseNode, TypeParseNode},
};

pub struct TupleTypeParseNode {
    pub fields: Vec<ParseNode<TypeParseNode>>,
}

impl TupleTypeParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|field| field.resolve_type(types))
            .collect();

        Type::Tuple(fields)
    }
}
