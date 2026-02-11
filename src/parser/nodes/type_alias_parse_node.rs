use crate::{
    checker::TypeResolver,
    parser::{Identified, IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct TypeAliasParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: ParseNode<TypeParseNode>,
}

impl TypeAliasParseNode {
    pub fn resolve_types(&self, types: &mut TypeResolver) {
        let resolved_type = self.type_def.value.resolve_type(types);
        types.resolve(self.id(), resolved_type);
    }
}

impl Identified for TypeAliasParseNode {
    fn id(&self) -> &String {
        &self.identifier.value.0
    }
}
