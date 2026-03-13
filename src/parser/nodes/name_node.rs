use crate::parser::Node;

pub struct NameNode(pub String);

pub trait Named {
    fn id(&self) -> &String;
}

impl Named for Node<NameNode> {
    fn id(&self) -> &String {
        &self.value.0
    }
}
