use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeResolver},
    parser::{Identified, IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct ParameterParseNode {
    identifier: ParseNode<IdentifierParseNode>,
    type_def: Option<ParseNode<TypeParseNode>>,
    resolved_type: OnceCell<Type>,
}

impl ParameterParseNode {
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

    pub fn get_type(&self, types: &TypeResolver) -> &Type {
        self.resolved_type.get_or_init(|| self.resolve_type(types))
    }

    fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.resolve_type(types),
            None => Type::Error,
        }
    }
}

impl Identified for ParameterParseNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
