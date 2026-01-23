use crate::{
    lexer::KeywordToken,
    parser::{ParseNode, ParseNodeVec, TokenSpan, Traverse},
};

#[derive(Debug)]
pub enum TypeDefinitionParseNode {
    Primitive(KeywordToken),
    User(UserDefinedTypeParseNode),
}

impl Traverse for TypeDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        match self {
            TypeDefinitionParseNode::Primitive(_) => {}
            TypeDefinitionParseNode::User(node) => node.traverse(visit),
        }
    }
}

#[derive(Debug)]
pub struct UserDefinedTypeParseNode {
    pub identifier: ParseNode<String>,
    pub generic_params: Option<ParseNodeVec<TypeDefinitionParseNode>>,
}

impl Traverse for UserDefinedTypeParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        if let Some(generics) = self.generic_params.as_ref() {
            visit(generics.span);
            for generic in generics.value.iter() {
                generic.traverse(visit);
            }
        }
    }
}
