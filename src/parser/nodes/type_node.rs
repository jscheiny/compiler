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
            Self::Primitive(_) => {}   // Primitive types are always valid
            Self::UserDefined(_) => {} // This should already be checked by get_type
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> Type {
        match self {
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(function) => Type::Function(function.get_type(types).clone()),
            Self::Tuple(tuple_type) => tuple_type.get_type(types).clone(),
            Self::UserDefined(identifier) => match types.get_type_ref(identifier) {
                Some(resolved_type) => resolved_type,
                None => {
                    println!("Type error: Unknown type `{}`", identifier);
                    Type::Error
                }
            },
        }
    }
}
