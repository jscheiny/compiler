use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{InterfaceType, Scope},
    parser::{MethodSignatureNode, NameNode, NodeVec},
};

pub struct InterfaceNode {
    pub name: NameNode,
    pub method_signatures: NodeVec<MethodSignatureNode>,
    resolved_type: OnceCell<Rc<InterfaceType>>,
}

impl InterfaceNode {
    pub fn new(name: NameNode, method_signatures: NodeVec<MethodSignatureNode>) -> Self {
        Self {
            name,
            method_signatures,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let mut method_names = HashSet::new();
        for method in self.method_signatures.iter() {
            let name = &method.signature.name;
            if !method_names.insert(&name.value) {
                scope.source.print_error(
                    name.span,
                    &format!("Duplicate method signature `{}`", name),
                    &format!("a method of `{}` already exists with this name", self.name),
                );
            }
        }

        scope
    }

    pub fn get_type(self: &Rc<Self>) -> Rc<InterfaceType> {
        self.resolved_type
            .get_or_init(|| InterfaceType::from(self.clone()))
            .clone()
    }
}
