use crate::{
    checker::{EnumType, FunctionType, StructType},
    parser::PrimitiveType,
};

#[derive(Clone)]
pub enum Type {
    Alias(Box<Type>),
    Enum(EnumType),
    Function(FunctionType),
    Primitive(PrimitiveType),
    Reference(usize),
    Struct(StructType),
    Tuple(Vec<Type>),
    Error,
}
