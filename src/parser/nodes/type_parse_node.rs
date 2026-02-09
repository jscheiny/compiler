use crate::{
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
