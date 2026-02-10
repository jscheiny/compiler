use crate::{
    checker::{TypeResolver, ResolveType, Type},
    lexer::KeywordToken,
    parser::{FunctionTypeParseNode, TokenSpan, Traverse, TupleTypeParseNode},
};

pub enum TypeParseNode {
    Primitive(KeywordToken), // TODO make a primitive type enum
    UserDefined(String),
    Function(FunctionTypeParseNode),
    Tuple(TupleTypeParseNode),
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

impl ResolveType for TypeParseNode {
    fn resolve_types(&self, types: &TypeResolver) -> Type {
        match self {
            TypeParseNode::Primitive(primitive) => {
                todo!("Primitive type resolution not implemented")
            }
            TypeParseNode::UserDefined(identifier) => {
                Type::Reference(types.get_reference(identifier))
            }
            TypeParseNode::Function(function_type) => function_type.resolve_types(types),
            TypeParseNode::Tuple(tuple_type) => tuple_type.resolve_types(types),
        }
    }
}
