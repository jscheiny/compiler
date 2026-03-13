use crate::{
    checker::{Scope, Type},
    parser::{InterfaceImplementationNode, MethodNode, Named},
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

impl Named for ImplementationEntryNode {
    fn name(&self) -> &String {
        match self {
            Self::Method(node) => node.name(),
            Self::Interface(node) => node.name(),
        }
    }
}
