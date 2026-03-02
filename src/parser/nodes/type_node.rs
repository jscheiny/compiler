use crate::{
    checker::{Scope, Type},
    parser::{FunctionTypeNode, Identified, IdentifierNode, Node, PrimitiveType, TupleTypeNode},
};

pub enum TypeNode {
    Array(Box<TypeNode>),
    Function(FunctionTypeNode),
    Primitive(PrimitiveType),
    Tuple(TupleTypeNode),
    UserDefined(Node<IdentifierNode>),
    Void,
}

impl TypeNode {
    pub fn get_type(&self, scope: &Scope) -> Type {
        match self {
            Self::Array(element_type) => Type::Array(Box::new(element_type.get_type(scope))),
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(function) => Type::Function(function.get_type(scope).clone()),
            Self::Tuple(tuple_type) => tuple_type.get_type(scope).clone(),
            Self::UserDefined(identifier) => match scope.get_type_index(identifier.id()) {
                Some(index) => Type::Reference(index),
                None => {
                    scope.source.print_error(
                        identifier.span,
                        &format!("Unknown type `{}`", identifier.id()),
                        "could not find this type",
                    );
                    Type::Error
                }
            },
            Self::Void => Type::Void,
        }
    }
}
