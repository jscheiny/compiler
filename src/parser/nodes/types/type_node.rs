use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{Type, TypeParameterMap, Types},
    parser::{FunctionTypeNode, PrimitiveType, TokenSpan, TupleTypeNode, UserDefinedTypeNode},
};

pub enum TypeNode {
    Array(Box<TypeNode>),
    Function(FunctionTypeNode),
    Primitive(PrimitiveType),
    ResultType(TokenSpan),
    SelfType(TokenSpan),
    Tuple(TupleTypeNode),
    UserDefined(UserDefinedTypeNode),
    Void,
}

pub type VisitedTypes = Option<Rc<RefCell<HashSet<usize>>>>;

impl TypeNode {
    pub fn get_type(
        &self,
        types: &impl Types,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Type {
        match self {
            Self::Array(node) => Type::Array(Box::new(node.get_type(types, type_params, visited))),
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(node) => Type::Function(node.get_type(types, type_params, visited)),
            Self::ResultType(span) => get_result_type(types, *span),
            Self::SelfType(span) => get_self_type(types, *span),
            Self::Tuple(node) => node.get_type(types, type_params, visited),
            Self::UserDefined(node) => node.get_type(types, type_params, visited),
            Self::Void => Type::Void,
        }
    }
}

fn get_result_type(types: &impl Types, span: TokenSpan) -> Type {
    let result_type = types.get_return_type();
    if let Some(result_type) = result_type {
        result_type
    } else {
        types.print_error(
            span,
            "`Result` type not available outside of function bodies",
            "cannot use type `Result` here",
        );
        Type::Error
    }
}

fn get_self_type(types: &impl Types, span: TokenSpan) -> Type {
    let self_type = types.get_self_type();
    if let Some(self_type) = self_type {
        self_type
    } else {
        types.print_error(
            span,
            "`Self` type not available outside of struct or enum",
            "cannot use type `Self` here",
        );
        Type::Error
    }
}
