use crate::parser::Node;

pub struct IdentifierNode(pub String);

pub trait Identified {
    fn id(&self) -> &String;
}

impl Identified for Node<IdentifierNode> {
    fn id(&self) -> &String {
        &self.value.0
    }
}
