use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    checker::{InterfaceType, Scope, Types},
    parser::{FunctionSignatureNode, NameNode, NodeVec},
};

pub struct InterfaceNode {
    pub name: NameNode,
    method_signatures: NodeVec<FunctionSignatureNode>,
    resolved_type: OnceCell<Rc<InterfaceType>>,
}

impl InterfaceNode {
    pub fn new(name: NameNode, method_signatures: NodeVec<FunctionSignatureNode>) -> Self {
        Self {
            name,
            method_signatures,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let mut method_names = HashSet::new();
        for method in self.method_signatures.iter() {
            if !method_names.insert(&method.name.value) {
                scope.source.print_error(
                    method.name.span,
                    &format!("Duplicate method signature `{}`", method.name),
                    &format!("a method of `{}` already exists with this name", self.name),
                );
            }
        }

        scope
    }

    pub fn get_type(&self, types: &impl Types) -> Rc<InterfaceType> {
        self.resolved_type
            .get_or_init(|| self.init_type(types))
            .clone()
    }

    fn init_type(&self, types: &impl Types) -> Rc<InterfaceType> {
        let mut methods = HashMap::new();
        for method_signature in self.method_signatures.iter() {
            let name = method_signature.name.clone();
            let method = method_signature.get_type(types).clone();
            methods.entry(name).or_insert(method);
        }

        Rc::new(InterfaceType {
            name: self.name.clone(),
            methods,
        })
    }
}
