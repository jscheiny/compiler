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
