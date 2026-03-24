use crate::{
    checker::Scope,
    parser::{ImplementationType, InterfaceImplementationNode, MethodNode},
};

pub enum ImplementationEntryNode {
    Method(Box<MethodNode>),
    Interface(InterfaceImplementationNode),
}

impl ImplementationEntryNode {
    pub fn check(&self, scope: Box<Scope>, self_type: &ImplementationType) -> Box<Scope> {
        match self {
            Self::Method(node) => node.check(scope),
            Self::Interface(node) => node.check(scope, self_type),
        }
    }
}
