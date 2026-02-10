use crate::{
    checker::{EnumType, FunctionType, StructType},
    parser::PrimitiveType,
};

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
