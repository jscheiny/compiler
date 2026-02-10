use crate::{
    checker::{Type, TypeResolver},
    parser::{IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct EnumVariantParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl EnumVariantParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Option<Type> {
        self.type_def
            .as_ref()
            .map(|ty| ty.value.resolve_type(types))
    }
}
