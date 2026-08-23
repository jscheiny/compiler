use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    checker::{InterfaceType, MethodType, Scope, Types},
    parser::{MethodSignatureNode, NameNode, NodeVec},
};

pub struct InterfaceNode {
    pub name: NameNode,
    method_signatures: NodeVec<MethodSignatureNode>,
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

    pub fn get_type(&self, types: &impl Types) -> Rc<InterfaceType> {
        self.resolved_type
            .get_or_init(|| self.init_type(types))
            .clone()
    }

    fn init_type(&self, types: &impl Types) -> Rc<InterfaceType> {
        let mut methods = HashMap::new();
        for method in self.method_signatures.iter() {
            let name = method.signature.name.clone();
            methods.entry(name).or_insert_with(|| MethodType {
                instance_kind: method.instance_kind,
                function_type: method.signature.get_type(types).clone(),
            });
        }

        Rc::new(InterfaceType {
            name: self.name.clone(),
            methods,
        })
    }
}
