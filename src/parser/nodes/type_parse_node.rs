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
    pub fn get_type(&self, types: &TypeResolver) -> Type {
        match self {
            TypeParseNode::Primitive(primitive) => Type::Primitive(*primitive),
            TypeParseNode::Function(function) => Type::Function(function.get_type(types).clone()),
            TypeParseNode::Tuple(tuple_type) => tuple_type.get_type(types).clone(),
            TypeParseNode::UserDefined(identifier) => types.get_type_ref(identifier),
        }
    }
}
