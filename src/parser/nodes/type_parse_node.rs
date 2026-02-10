use crate::{
    checker::{Type, TypeResolver},
    parser::{FunctionTypeParseNode, PrimitiveType, TupleTypeParseNode},
};

pub enum TypeParseNode {
    Function(FunctionTypeParseNode),
    Primitive(PrimitiveType),
    Tuple(TupleTypeParseNode),
    UserDefined(String),
}

impl TypeParseNode {
    pub fn resolve_type(&self, types: &TypeResolver) -> Type {
        match self {
            TypeParseNode::Primitive(primitive) => Type::Primitive(*primitive),
            TypeParseNode::Function(function_type) => function_type.resolve_type(types),
            TypeParseNode::Tuple(tuple_type) => tuple_type.resolve_type(types),
            TypeParseNode::UserDefined(identifier) => types.get_type_ref(identifier),
        }
    }
}
