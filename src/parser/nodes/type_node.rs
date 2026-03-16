use crate::{
    checker::{Scope, Type, TypeParameterMap},
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
    pub fn get_type(&self, scope: &Scope, type_params: Option<&TypeParameterMap>) -> Type {
        match self {
            Self::Array(node) => Type::Array(Box::new(node.get_type(scope, type_params))),
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(node) => Type::Function(node.get_type(scope, type_params)),
            Self::Tuple(node) => node.get_type(scope, type_params),
            Self::UserDefined(node) => node.get_type(scope, type_params),
            Self::Void => Type::Void,
        }
    }
}
