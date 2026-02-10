use std::collections::HashMap;

use crate::checker::{FunctionType, Type};

pub struct StructType {
    pub members: HashMap<String, StructMember>,
}

pub struct StructMember {
    pub public: bool,
    pub member_type: StructMemberType,
}

pub enum StructMemberType {
    Field(Type),
    Method(FunctionType),
}
