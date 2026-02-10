use crate::{
    checker::{Type, TypeResolver},
    parser::{FunctionTypeParseNode, PrimitiveType, TokenSpan, Traverse, TupleTypeParseNode},
};

pub enum TypeParseNode {
    Function(FunctionTypeParseNode),
    Primitive(PrimitiveType),
    Tuple(TupleTypeParseNode),
    UserDefined(String),
}

impl Traverse for TypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Function(node) => node.traverse(visit),
            Self::Tuple(node) => node.traverse(visit),
            Self::Primitive(_) | Self::UserDefined(_) => {}
        }
    }
}

impl TypeParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self {
            TypeParseNode::Primitive(primitive) => Type::Primitive(*primitive),
            TypeParseNode::Function(function_type) => function_type.resolve_type(types),
            TypeParseNode::Tuple(tuple_type) => tuple_type.resolve_type(types),
            TypeParseNode::UserDefined(identifier) => Type::Reference(types.get_ref(identifier)),
        }
    }
}
