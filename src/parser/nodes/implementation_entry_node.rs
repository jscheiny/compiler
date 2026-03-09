use crate::{
    checker::{Scope, Type},
    parser::{Identified, InterfaceImplementationNode, MethodNode},
};

pub enum ImplementationEntryNode {
    Method(Box<MethodNode>),
    Interface(InterfaceImplementationNode),
}

impl ImplementationEntryNode {
    pub fn check(&self, scope: Box<Scope>, self_type: &Type) -> Box<Scope> {
        match self {
            Self::Method(node) => node.check(scope),
            Self::Interface(node) => node.check(scope, self_type),
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
