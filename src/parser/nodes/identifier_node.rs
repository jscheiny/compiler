use crate::parser::ParseNode;

pub struct IdentifierNode(pub String);

pub trait Identified {
    fn id(&self) -> &String;
}

impl Identified for ParseNode<IdentifierNode> {
    fn id(&self) -> &String {
        &self.value.0
    }
}
