use crate::{
    checker::{Type, TypeResolver},
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

impl TypeParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self {
            TypeParseNode::Primitive(primitive) => {
                todo!("Primitive type resolution not implemented")
            }
            TypeParseNode::UserDefined(identifier) => {
                Type::Reference(types.get_reference(identifier))
            }
            TypeParseNode::Function(function_type) => function_type.resolve_type(types),
            TypeParseNode::Tuple(tuple_type) => tuple_type.resolve_type(types),
        }
    }
}
