use std::collections::HashMap;

use crate::checker::{FunctionType, Type};

#[derive(Clone, Debug)]
pub struct StructType {
    pub members: HashMap<String, StructMember>,
}

#[derive(Clone, Debug)]
pub struct StructMember {
    pub public: bool,
    pub member_type: StructMemberType,
}

#[derive(Clone, Debug)]
pub enum StructMemberType {
    Field(Type),
    Method(FunctionType),
}
