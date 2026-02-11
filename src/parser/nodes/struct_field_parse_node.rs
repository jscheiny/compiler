use crate::{
    checker::{StructMember, StructMemberType, Type, TypeResolver},
    parser::{Identified, IdentifierParseNode, ParseNode, TypeParseNode},
};

pub struct StructFieldParseNode {
    pub public: bool,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
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

impl Identified for StructFieldParseNode {
    fn id(&self) -> &String {
        &self.identifier.value.0
    }
}
