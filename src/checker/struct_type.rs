use std::collections::HashMap;

use crate::checker::{DuplicateMemberName, FunctionType, Type, TypeError, TypeResolver};

#[derive(Default, Clone)]
pub struct StructType {
    pub members: HashMap<String, StructMember>,
}

impl StructType {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_member(
        &mut self,
        identifier: String,
        container_name: &str,
        member: StructMember,
        types: &mut TypeResolver,
    ) {
        if self.members.contains_key(&identifier) {
            types.push_error(TypeError::DuplicateMemberName(DuplicateMemberName {
                container_name: container_name.to_owned(),
                container_type: "struct".to_owned(),
                member_name: identifier,
                member_type: "member".to_owned(),
            }));
        } else {
            self.members.insert(identifier, member);
        }
    }
}

#[derive(Clone)]
pub struct StructMember {
    pub public: bool,
    pub member_type: StructMemberType,
}

#[derive(Clone)]
pub enum StructMemberType {
    Field(Type),
    Method(FunctionType),
}
