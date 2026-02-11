use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct EnumVariantParseNode {
    identifier: ParseNode<IdentifierParseNode>,
    type_def: Option<ParseNode<TypeParseNode>>,
    resolved_type: OnceCell<Option<Type>>,
}

impl EnumVariantParseNode {
    pub fn new(
        identifier: ParseNode<IdentifierParseNode>,
        type_def: Option<ParseNode<TypeParseNode>>,
    ) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> Option<&Type> {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(types))
            .as_ref()
    }

    fn get_type_impl(&self, types: &TypeResolver) -> Option<Type> {
        self.type_def.as_ref().map(|ty| ty.get_type(types))
    }
}

impl Identified for EnumVariantParseNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
