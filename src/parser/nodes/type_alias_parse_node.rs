use crate::{
    checker::TypeResolver,
    parser::{IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct TypeAliasParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: ParseNode<TypeParseNode>,
}

impl TypeAliasParseNode {
    pub fn declare_type(&self, types: &mut TypeResolver) {
        types.declare(self.identifier());
    }

    pub fn resolve_types(&self, types: &mut TypeResolver) {
        let resolved_type = self.type_def.value.resolve_type(types);
        types.resolve(self.identifier(), resolved_type);
    }

    pub fn identifier(&self) -> &String {
        &self.identifier.value.0
    }
}
