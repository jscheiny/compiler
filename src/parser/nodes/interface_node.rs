use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    checker::{InterfaceType, Scope},
    parser::{FunctionSignatureNode, NameNode, Named, Node, NodeVec},
};

pub struct InterfaceNode {
    pub name: Node<NameNode>,
    method_signatures: NodeVec<FunctionSignatureNode>,
    resolved_type: OnceCell<Rc<InterfaceType>>,
}

impl InterfaceNode {
    pub fn new(name: Node<NameNode>, method_signatures: NodeVec<FunctionSignatureNode>) -> Self {
        Self {
            name,
            method_signatures,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let mut method_names = HashSet::new();
        for method in self.method_signatures.iter() {
            if !method_names.insert(method.name()) {
                scope.source.print_error(
                    method.name.span,
                    &format!("Duplicate method signature `{}`", method.name()),
                    &format!(
                        "a method of `{}` already exists with this name",
                        self.name()
                    ),
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
            let id = method_signature.name().clone();
            let method = method_signature.get_type(scope).clone();
            methods.entry(id).or_insert(method);
        }

        Rc::new(InterfaceType {
            name: self.name().clone(),
            methods,
        })
    }
}

impl Named for InterfaceNode {
    fn name(&self) -> &String {
        self.name.name()
    }
}
