use crate::parser::Node;

pub struct NameNode(pub String);

pub trait Named {
    fn name(&self) -> &String;
}

impl Named for Node<NameNode> {
    fn name(&self) -> &String {
        &self.value.0
    }
}
