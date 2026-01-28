use crate::{
    lexer::KeywordToken,
    parser::{IdentifierParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse},
};

#[derive(Debug)]
pub enum TypeParseNode {
    Primitive(KeywordToken),
    UserDefined(UserDefinedTypeParseNode),
}

impl Traverse for TypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Primitive(_) => {}
            Self::UserDefined(node) => node.traverse(visit),
        }
    }
}

#[derive(Debug)]
pub struct UserDefinedTypeParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub generic_params: Option<ParseNodeVec<TypeParseNode>>,
}

impl Traverse for UserDefinedTypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("UserDefinedType.identifier", self.identifier.span);
        if let Some(generics) = self.generic_params.as_ref() {
            visit("UserDefinedType.generics", generics.span);
            for generic in generics.value.iter() {
                generic.traverse("UserDefinedType.generic", visit);
            }
        }
    }
}
