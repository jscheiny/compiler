use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct TypeAliasParseNode {
    identifier: ParseNode<IdentifierParseNode>,
    type_def: ParseNode<TypeParseNode>,
    resolved_type: OnceCell<Type>,
}

impl TypeAliasParseNode {
    pub fn new(
        identifier: ParseNode<IdentifierParseNode>,
        type_def: ParseNode<TypeParseNode>,
    ) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> &Type {
        self.resolved_type
            .get_or_init(|| self.type_def.get_type(types))
    }
}

impl Identified for TypeAliasParseNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
