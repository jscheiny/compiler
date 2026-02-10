use crate::{
    checker::{ResolveType, TypeResolver},
    parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode},
};

pub struct TypeAliasParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: ParseNode<TypeParseNode>,
}

impl Traverse for TypeAliasParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("TypeAlias.identifier", self.identifier.span);
        self.type_def.traverse("TypeAlias.identifer", visit);
    }
}

impl TypeAliasParseNode {
    pub fn register_type(&self, types: &mut TypeResolver) {
        let resolved_type = self.type_def.value.resolve_type(types);
        types.insert(&self.identifier.value.0, resolved_type);
    }
}
