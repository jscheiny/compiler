use crate::{
    checker::{Type, TypeResolver},
    lexer::SourceCode,
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
    pub fn get_type(&self, types: &TypeResolver, source: &SourceCode) -> Type {
        match self {
            Self::Array(element_type) => {
                Type::Array(Box::new(element_type.get_type(types, source)))
            }
            Self::Primitive(primitive) => Type::Primitive(*primitive),
            Self::Function(function) => Type::Function(function.get_type(types, source).clone()),
            Self::Tuple(tuple_type) => tuple_type.get_type(types, source).clone(),
            Self::UserDefined(identifier) => match types.get_ref(identifier.id()) {
                Some(resolved_type) => Type::Reference(resolved_type),
                None => {
                    source.print_type_error(
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
