use crate::{
    checker::{Type, TypeResolver},
    parser::{FunctionTypeNode, PrimitiveType, TupleTypeNode},
};

pub enum TypeNode {
    Array(Box<TypeNode>),
    Function(FunctionTypeNode),
    Primitive(PrimitiveType),
    Tuple(TupleTypeNode),
    UserDefined(String),
    Void,
}

impl TypeNode {
    pub fn get_type(&self, types: &TypeResolver) -> Type {
        match self {
            Self::Array(element_type) => Type::Array(Box::new(element_type.get_type(types))),
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(function) => Type::Function(function.get_type(types).clone()),
            Self::Tuple(tuple_type) => tuple_type.get_type(types).clone(),
            Self::UserDefined(identifier) => match types.get_ref(identifier) {
                Some(resolved_type) => Type::Reference(resolved_type),
                None => {
                    println!("Type error: Unknown type `{}`", identifier);
                    Type::Error
                }
            },
            Self::Void => Type::Void,
        }
    }
}
