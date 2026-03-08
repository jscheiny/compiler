use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{FunctionType, InterfaceType, Scope, Type},
    parser::{Identified, ImplementationEntryNode, MethodNode, Node},
};

pub struct ImplementationNode {
    pub entries: Vec<Node<ImplementationEntryNode>>,
    implemented_interfaces: OnceCell<HashSet<usize>>,
}

#[derive(Clone, Copy)]
pub enum ImplementationNodeType {
    Struct,
    Enum,
}

impl ImplementationNode {
    pub fn new(entries: Vec<Node<ImplementationEntryNode>>) -> Self {
        Self {
            entries,
            implemented_interfaces: OnceCell::new(),
        }
    }

    pub fn check(
        &self,
        mut scope: Box<Scope>,
        implementation_type: ImplementationNodeType,
        container_name: &str,
        mut scope_names: HashSet<String>,
    ) -> Box<Scope> {
        for entry in self.entries.iter() {
            match &entry.value {
                ImplementationEntryNode::Method(method) => check_method_duplicate(
                    method,
                    &mut scope,
                    implementation_type,
                    container_name,
                    &mut scope_names,
                ),
                ImplementationEntryNode::Interface(_) => {
                    // TODO Handle duplication checking for interface implementations
                }
            };
        }

        for entry in self.entries.iter() {
            scope = entry.check(scope);
        }

        scope
    }

    // TODO get a better API for this
    pub fn get_methods(&self, scope: &Scope) -> Vec<(String, bool, Rc<FunctionType>)> {
        let mut methods = vec![];
        for entry in self.entries.iter() {
            match &entry.value {
                ImplementationEntryNode::Method(method) => {
                    methods.push((
                        method.id().clone(),
                        method.public,
                        method.function.get_type(scope).clone(),
                    ));
                }
                ImplementationEntryNode::Interface(implementation) => {
                    let interface_type = scope
                        .get_type_index(implementation.id())
                        .map(Type::Reference)
                        .map(|t| t.as_deref(scope));
                    if let Some(Type::Interface(interface_type)) = interface_type {
                        for (name, function_type) in interface_type.methods.iter() {
                            methods.push((name.clone(), true, function_type.clone()));
                        }
                    }
                }
            }
        }

        methods
    }

    pub fn implements(&self, scope: &Scope, interface_type: &Rc<InterfaceType>) -> bool {
        let index = scope.global().get_type_index(&interface_type.identifier);
        match index {
            Some(index) => self
                .implemented_interfaces
                .get_or_init(|| self.init_implemented_interfaces(scope))
                .contains(&index),
            None => false,
        }
    }

    fn init_implemented_interfaces(&self, scope: &Scope) -> HashSet<usize> {
        let mut result = HashSet::new();
        for entry in self.entries.iter() {
            if let ImplementationEntryNode::Interface(node) = &entry.value {
                let index = scope.get_type_index(node.id());
                if let Some(index) = index {
                    result.insert(index);
                }
            }
        }

        result
    }
}

fn check_method_duplicate(
    method: &MethodNode,
    scope: &mut Scope,
    implementation_type: ImplementationNodeType,
    container_name: &str,
    scope_names: &mut HashSet<String>,
) {
    if scope_names.contains(method.id()) {
        print_duplicate_member_error(scope, implementation_type, container_name, method);
    } else {
        let method_type = Type::Function(method.function.get_type(scope).clone());
        scope.add_value(method.id(), method_type);
        scope_names.insert(method.id().clone());
    }
}

fn print_duplicate_member_error(
    scope: &Scope,
    implementation_type: ImplementationNodeType,
    container_name: &str,
    method: &MethodNode,
) {
    use ImplementationNodeType as I;
    let container_type = match implementation_type {
        I::Enum => "enum",
        I::Struct => "struct",
    };
    scope.source.print_error(
        method.function.signature.identifier.span,
        &format!("Duplicate {} member `{}`", container_type, method.id()),
        &format!(
            "{} `{}` already contains a {} with this name",
            container_type,
            container_name,
            match implementation_type {
                I::Enum => "variant or method",
                I::Struct => "field or method",
            }
        ),
    );
}
