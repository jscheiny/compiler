use crate::{
    lexer::KeywordToken,
    parser::{IdentifierParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse},
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

pub struct FunctionTypeParseNode {
    pub parameters: ParseNodeVec<TypeParseNode>,
    pub return_type: Box<ParseNode<TypeParseNode>>,
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

pub struct TupleTypeParseNode {
    pub members: Vec<ParseNode<TypeParseNode>>,
}

impl Traverse for TupleTypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for member in self.members.iter() {
            member.traverse("TupleType.member", visit);
        }
    }
}

pub struct TypeAliasParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: ParseNode<TypeParseNode>,
}

impl Traverse for TypeAliasParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("TypeAlias.identifier", self.identifier.span);
        self.type_def.traverse("TypeAlias.type", visit);
    }
}
