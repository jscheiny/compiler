use crate::{
    checker::{Type, TypeResolver},
    parser::{IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct ParameterParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl ParameterParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.value.resolve_type(types),
            None => Type::Error,
        }
    }
}
