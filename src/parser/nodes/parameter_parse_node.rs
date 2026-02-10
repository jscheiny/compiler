use crate::{
    checker::{Type, TypeResolver},
    parser::{IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct ParameterParseNode {
    identifier: ParseNode<IdentifierParseNode>,
    type_def: Option<ParseNode<TypeParseNode>>,
    resolved_type: Option<Type>,
}

impl ParameterParseNode {
    pub fn new(
        identifier: ParseNode<IdentifierParseNode>,
        type_def: Option<ParseNode<TypeParseNode>>,
    ) -> Self {
        Self {
            identifier,
            type_def,
            resolved_type: None,
        }
    }

    pub fn get_type(&mut self, types: &TypeResolver) -> Type {
        match self.resolved_type.as_ref() {
            Some(resolved_type) => resolved_type.clone(),
            None => {
                let resolved_type = self.resolve_type(types);
                let result = resolved_type.clone();
                self.resolved_type = Some(resolved_type);
                result
            }
        }
    }

    fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.value.resolve_type(types),
            None => Type::Error,
        }
    }

    pub fn identifier(&self) -> &String {
        &self.identifier.value.0
    }
}
