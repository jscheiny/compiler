use std::collections::HashMap;

use crate::checker::{FunctionType, Type};

#[derive(Clone, Debug)]
pub struct StructType {
    pub identifier: String,
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

impl StructMemberType {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Field(field_type) => field_type.clone(),
            Self::Method(function_type) => Type::Function(function_type.clone()),
        }
    }

    pub fn as_static_type(self, self_type: Type) -> Type {
        match self {
            Self::Field(field_type) => Type::Function(FunctionType::new(self_type, field_type)),
            Self::Method(function_type) => function_type.as_static_method(self_type),
        }
    }
}
