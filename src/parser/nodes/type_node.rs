use crate::{
    checker::{Generics, Scope, Type},
    parser::{FunctionTypeNode, PrimitiveType, TupleTypeNode, UserDefinedTypeNode},
};

pub enum TypeNode {
    Array(Box<TypeNode>),
    Function(FunctionTypeNode),
    Primitive(PrimitiveType),
    Tuple(TupleTypeNode),
    UserDefined(UserDefinedTypeNode),
    Void,
}

impl TypeNode {
    pub fn get_type(&self, scope: &Scope, generics: Generics<'_>) -> Type {
        match self {
            Self::Array(node) => Type::Array(Box::new(node.get_type(scope, generics))),
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(node) => Type::Function(node.get_type(scope, generics)),
            Self::Tuple(node) => node.get_type(scope, generics),
            Self::UserDefined(node) => node.get_type(scope),
            Self::Void => Type::Void,
        }
    }
}
