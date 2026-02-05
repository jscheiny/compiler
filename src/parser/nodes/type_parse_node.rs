use crate::{
    lexer::KeywordToken,
    parser::{ParseNode, ParseNodeVec, TokenSpan, Traverse},
};

pub enum TypeParseNode {
    Primitive(KeywordToken), // TODO make a primitive type enum
    UserDefined(String),
    Function(FunctionTypeParseNode),
}

impl Traverse for TypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Primitive(_) | Self::UserDefined(_) => {}
            Self::Function(node) => node.traverse(visit),
        }
    }
}

pub struct FunctionTypeParseNode {
    parameters: Box<ParseNodeVec<TypeParseNode>>,
    return_type: Box<ParseNode<TypeParseNode>>,
}

impl Traverse for FunctionTypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("FunctionType.parameters", self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse("FunctionType.parameter", visit);
        }
        self.return_type.traverse("FunctionType.return_type", visit);
    }
}
