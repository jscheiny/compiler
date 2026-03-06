use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    checker::{InterfaceType, Scope},
    parser::{FunctionSignatureNode, Identified, IdentifierNode, Node, NodeVec},
};

pub struct InterfaceNode {
    pub identifier: Node<IdentifierNode>,
    method_signatures: NodeVec<FunctionSignatureNode>,
    resolved_type: OnceCell<Rc<InterfaceType>>,
}

impl InterfaceNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        method_signatures: NodeVec<FunctionSignatureNode>,
    ) -> Self {
        Self {
            identifier,
            method_signatures,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let mut method_names = HashSet::new();
        for method in self.method_signatures.iter() {
            if !method_names.insert(method.id()) {
                scope.source.print_error(
                    method.identifier.span,
                    &format!("Duplicate method signature `{}`", method.id()),
                    &format!("a method of `{}` already exists with this name", self.id()),
                );
            }
        }

        scope
    }

    pub fn get_type(&self, scope: &Scope) -> Rc<InterfaceType> {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(scope))
            .clone()
    }

    fn get_type_impl(&self, scope: &Scope) -> Rc<InterfaceType> {
        let mut methods = HashMap::new();
        for method_signature in self.method_signatures.iter() {
            let id = method_signature.id().clone();
            let method = method_signature.get_type(scope).clone();
            methods.entry(id).or_insert(method);
        }

        Rc::new(InterfaceType {
            identifier: self.id().clone(),
            methods,
        })
    }
}

impl Identified for InterfaceNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
