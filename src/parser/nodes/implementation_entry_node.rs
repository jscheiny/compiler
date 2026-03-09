use crate::{
    checker::Scope,
    parser::{Identified, InterfaceImplementationNode, MethodNode},
};

pub enum ImplementationEntryNode {
    Method(Box<MethodNode>),
    Interface(InterfaceImplementationNode),
}

impl ImplementationEntryNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        match self {
            Self::Method(node) => node.check(scope),
            Self::Interface(node) => node.check(scope),
        }
    }
}

impl Identified for ImplementationEntryNode {
    fn id(&self) -> &String {
        match self {
            Self::Method(node) => node.id(),
            Self::Interface(node) => node.id(),
        }
    }
}
