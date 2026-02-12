use crate::{
    checker::{EnumType, FunctionType, StructType},
    parser::PrimitiveType,
};

#[derive(Clone, Debug)]
pub enum Type {
    Enum(EnumType),
    Function(FunctionType),
    Primitive(PrimitiveType),
    Reference(usize),
    Struct(StructType),
    Tuple(Vec<Type>),
    Type(RuntimeType),
    Error,
}

// TODO reconsider this name
#[derive(Clone, Debug)]
pub enum RuntimeType {
    Struct(StructType),
}

impl Type {
    pub fn is_primitive(&self, expected: PrimitiveType) -> bool {
        match self {
            Self::Primitive(primitive) => *primitive == expected,
            Self::Error => true,
            _ => false,
        }
    }
}
