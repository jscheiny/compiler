use crate::{
    checker::{Type, TypeResolver},
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

impl TupleTypeParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        let fields = self
            .fields
            .iter()
            .map(|p| p.value.resolve_type(types))
            .collect();

        Type::Tuple(fields)
    }
}
