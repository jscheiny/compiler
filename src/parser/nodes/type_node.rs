use crate::{
    checker::{Type, TypeResolver},
    parser::{FunctionTypeNode, PrimitiveType, TupleTypeNode},
};

pub enum TypeNode {
    Function(FunctionTypeNode),
    Primitive(PrimitiveType),
    Tuple(TupleTypeNode),
    UserDefined(String),
}

impl TypeNode {
    pub fn check(&self, types: &TypeResolver) {
        match self {
            Self::Function(node) => node.check(types),
            Self::Tuple(node) => node.check(types),
            Self::Primitive(_) => {}
            Self::UserDefined(identifier) => {
                // TODO this should check the current scope as well
                if !types.contains(identifier) {
                    println!("Type error: Unknown type `{}`", identifier);
                }
            }
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> Type {
        match self {
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(function) => Type::Function(function.get_type(types).clone()),
            Self::Tuple(tuple_type) => tuple_type.get_type(types).clone(),
            Self::UserDefined(identifier) => types.get_type_ref(identifier),
        }
    }
}
