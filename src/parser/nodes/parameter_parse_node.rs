use crate::{
    checker::{Type, TypeResolver},
    parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode},
};

pub struct ParameterParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl Traverse for ParameterParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Parameter.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("Parameter.type", visit);
        }
    }
}

impl ParameterParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self.type_def.as_ref() {
            Some(type_def) => type_def.value.resolve_type(types),
            None => Type::Error,
        }
    }
}
