use crate::{
    checker::{Scope, Type, TypeParameterMap},
    parser::{FunctionTypeNode, PrimitiveType, TokenSpan, TupleTypeNode, UserDefinedTypeNode},
};

pub enum TypeNode {
    Array(Box<TypeNode>),
    Function(FunctionTypeNode),
    Primitive(PrimitiveType),
    SelfType(TokenSpan),
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
            Self::SelfType(span) => get_self_type(scope, *span),
            Self::Tuple(node) => node.get_type(scope, type_params),
            Self::UserDefined(node) => node.get_type(scope, type_params),
            Self::Void => Type::Void,
        }
    }
}

fn get_self_type(scope: &Scope, span: TokenSpan) -> Type {
    let self_type = scope.get_self_type();
    if let Some(self_type) = self_type {
        self_type
    } else {
        scope.source.print_error(
            span,
            "Self type not available outside of struct or enum",
            "cannot use type `Self` here",
        );
        Type::Error
    }
}
