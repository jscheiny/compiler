use std::collections::HashMap;

use crate::checker::{DuplicateMemberName, FunctionType, Type, TypeError, TypeResolver};

#[derive(Default)]
pub struct EnumType {
    pub members: HashMap<String, EnumMember>,
}

impl EnumType {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_member(
        &mut self,
        identifier: &String,
        container_name: &str,
        member: EnumMember,
        types: &mut TypeResolver,
    ) {
        if self.members.contains_key(identifier) {
            types.push_error(TypeError::DuplicateMemberName(DuplicateMemberName {
                member_name: identifier.clone(),
                container_name: container_name.to_owned(),
                container_type: "enum".to_owned(),
            }));
        } else {
            self.members.insert(identifier.clone(), member);
        }
    }
}

pub enum EnumMember {
    Variant(Option<Type>),
    Method(EnumMethod),
}

pub struct EnumMethod {
    pub public: bool,
    pub function_type: FunctionType,
}
