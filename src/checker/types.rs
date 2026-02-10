use std::collections::HashMap;

use crate::parser::PrimitiveType;

#[derive(Clone, Copy)]
pub enum TypeReference {
    Resolved(usize),
    Unresolved,
}

pub enum Type {
    Alias(Box<Type>),
    Enum(EnumType),
    Function(FunctionType),
    Primitive(PrimitiveType),
    Reference(TypeReference),
    Struct(StructType),
    Tuple(Vec<Type>),
    Error,
}

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

pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Option<Box<Type>>,
}

pub struct EnumType {
    pub members: HashMap<String, EnumMember>,
}

pub enum EnumMember {
    Variant(Option<Type>),
    Method(FunctionType),
}
