use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum TypeReference {
    Resolved(usize),
    Unresolved,
}

pub enum Type {
    Error,
    Reference(TypeReference),
    Alias(Box<Type>),
    Struct(StructType),
    Enum(EnumType),
    Function(FunctionType),
    Tuple(Vec<Type>),
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
