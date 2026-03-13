use crate::parser::Node;

pub struct NameNode(pub String);

pub trait Identified {
    fn id(&self) -> &String;
}

impl Identified for Node<NameNode> {
    fn id(&self) -> &String {
        &self.value.0
    }
}
