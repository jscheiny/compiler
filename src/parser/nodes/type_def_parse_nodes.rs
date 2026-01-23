use crate::{
    lexer::KeywordToken,
    parser::{LocatedNode, LocatedNodeVec},
};

#[derive(Debug)]
pub enum TypeDefinitionParseNode {
    Primitive(KeywordToken),
    User(UserDefinedTypeParseNode),
}

#[derive(Debug)]
pub struct UserDefinedTypeParseNode {
    pub identifier: LocatedNode<String>,
    pub generic_params: Option<LocatedNodeVec<TypeDefinitionParseNode>>,
}
