use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{MethodType, Scope},
    parser::InterfaceNode,
};

pub struct InterfaceType {
    node: Rc<InterfaceNode>,
    methods: OnceCell<HashMap<String, MethodType>>,
}

impl InterfaceType {
    pub fn from(node: Rc<InterfaceNode>) -> Rc<Self> {
        Rc::new(Self {
            node,
            methods: OnceCell::new(),
        })
    }

    pub fn name(&self) -> &String {
        &self.node.name
    }

    pub fn complete(self: &Rc<Self>, scope: &Scope) {
        self.get_methods(scope);
    }

    pub fn get_method(self: &Rc<Self>, scope: &Scope, name: &String) -> Option<&MethodType> {
        self.get_methods(scope).get(name)
    }

    pub fn get_methods(self: &Rc<Self>, scope: &Scope) -> &HashMap<String, MethodType> {
        self.methods.get_or_init(|| self.init_methods(scope))
    }

    fn init_methods(&self, scope: &Scope) -> HashMap<String, MethodType> {
        let mut methods = HashMap::new();
        for method in self.node.method_signatures.iter() {
            let name = method.signature.name.clone();
            methods.entry(name).or_insert_with(|| MethodType {
                instance_kind: method.instance_kind,
                function_type: method.signature.get_type(scope).clone(),
            });
        }

        methods
    }
}
