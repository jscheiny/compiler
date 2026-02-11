use crate::{
    checker::{StructMember, StructMemberType, Type, TypeResolver},
    parser::{Identified, IdentifierNode, ParseNode, TypeNode},
};

pub struct StructFieldNode {
    pub public: bool,
    pub identifier: ParseNode<IdentifierNode>,
    pub type_def: Option<ParseNode<TypeNode>>,
}

impl StructFieldNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> StructMember {
        let member_type = match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(types),
            None => Type::Error,
        };

        StructMember {
            public: self.public,
            member_type: StructMemberType::Field(member_type),
        }
    }
}

impl Identified for StructFieldNode {
    fn id(&self) -> &String {
        &self.identifier.id()
    }
}
