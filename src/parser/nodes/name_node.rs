use std::fmt::Display;

use crate::parser::Node;

pub type NameNode = Node<String>;

pub trait Named {
    fn name(&self) -> &String;
}

impl Display for NameNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
