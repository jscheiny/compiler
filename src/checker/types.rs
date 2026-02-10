use std::collections::HashMap;

use crate::checker::TypeResolver;

pub trait ResolveType {
    fn resolve_types(&self, types: &TypeResolver) -> Type;
}

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
    pub declarations: HashMap<String, StructDeclaration>,
}

pub struct StructDeclaration {
    pub public: bool,
    pub declaration_type: StructDeclarationType,
}

pub enum StructDeclarationType {
    Member(Type),
    Method(FunctionType),
}

pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
}

pub struct EnumType {
    pub variants: HashMap<String, Type>,
}
