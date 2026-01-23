use crate::{
    lexer::KeywordToken,
    parser::{ParseNode, ParseNodeVec},
};

#[derive(Debug)]
pub enum TypeDefinitionParseNode {
    Primitive(KeywordToken),
    User(UserDefinedTypeParseNode),
}

#[derive(Debug)]
pub struct UserDefinedTypeParseNode {
    pub identifier: ParseNode<String>,
    pub generic_params: Option<ParseNodeVec<TypeDefinitionParseNode>>,
}
