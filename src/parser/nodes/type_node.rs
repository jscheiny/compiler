use crate::{
    checker::{Scope, Type},
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
    pub fn get_type(&self, scope: &Scope) -> Type {
        match self {
            Self::Array(node) => Type::Array(Box::new(node.get_type(scope))),
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(node) => Type::Function(node.get_type(scope)),
            Self::Tuple(node) => node.get_type(scope),
            Self::UserDefined(node) => node.get_type(scope),
            Self::Void => Type::Void,
        }
    }
}
