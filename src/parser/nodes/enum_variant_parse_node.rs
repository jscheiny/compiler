use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct EnumVariantParseNode {
    identifier: ParseNode<IdentifierParseNode>,
    type_def: Option<ParseNode<TypeParseNode>>,
}

impl EnumVariantParseNode {
    pub fn new(
        identifier: ParseNode<IdentifierParseNode>,
        type_def: Option<ParseNode<TypeParseNode>>,
    ) -> Self {
        Self {
            identifier,
            type_def,
        }
    }

    pub fn resolve_type(&self, types: &TypeResolver) -> Option<Type> {
        self.type_def
            .as_ref()
            .map(|ty| ty.value.resolve_type(types))
    }
}

impl Identified for EnumVariantParseNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
