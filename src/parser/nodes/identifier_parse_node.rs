use crate::parser::ParseNode;

pub struct IdentifierParseNode(pub String);

pub trait Identified {
    fn id(&self) -> &String;
}

impl<T: Identified> Identified for ParseNode<T> {
    fn id(&self) -> &String {
        self.value.id()
    }
}

impl Identified for ParseNode<IdentifierParseNode> {
    fn id(&self) -> &String {
        &self.value.0
    }
}
