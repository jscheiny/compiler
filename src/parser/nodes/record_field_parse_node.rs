use crate::{
    checker::{StructMember, StructMemberType, Type, TypeResolver},
    parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode},
};

pub struct StructFieldParseNode {
    pub public: bool,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl Traverse for StructFieldParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("StructField.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("StructField.type", visit);
        }
    }
}

impl StructFieldParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> StructMember {
        let member_type = match self.type_def.as_ref() {
            Some(type_def) => type_def.value.resolve_type(types),
            None => Type::Error,
        };

        StructMember {
            public: self.public,
            member_type: StructMemberType::Field(member_type),
        }
    }
}
