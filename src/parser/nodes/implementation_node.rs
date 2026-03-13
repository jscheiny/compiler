use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{FunctionType, InterfaceType, Scope, Type},
    parser::{FunctionNode, ImplementationEntryNode, InterfaceImplementationNode, Named, Node},
};

pub struct ImplementationNode {
    pub entries: Vec<Node<ImplementationEntryNode>>,
    implemented_interfaces: OnceCell<HashSet<usize>>,
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
        self_type: &Type,
        mut scope_names: HashSet<String>,
    ) -> Box<Scope> {
        let mut implemented_interfaces = HashSet::new();
        for entry in self.entries.iter() {
            match &entry.value {
                ImplementationEntryNode::Interface(interface) => check_duplicate_interface(
                    interface,
                    &mut scope,
                    self_type,
                    &mut scope_names,
                    &mut implemented_interfaces,
                ),
                ImplementationEntryNode::Method(method) => check_duplicate_method(
                    &method.function,
                    &mut scope,
                    self_type,
                    &mut scope_names,
                ),
            };
        }

        for entry in self.entries.iter() {
            scope = entry.check(scope, self_type);
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
                        method.name().clone(),
                        method.public,
                        method.function.get_type(scope).clone(),
                    ));
                }
                ImplementationEntryNode::Interface(implementation) => {
                    let interface_type = scope
                        .get_type_index(implementation.name())
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
                let index = scope.get_type_index(node.name());
                if let Some(index) = index {
                    result.insert(index);
                }
            }
        }

        result
    }
}

fn check_duplicate_interface(
    interface_implementation: &InterfaceImplementationNode,
    scope: &mut Scope,
    self_type: &Type,
    scope_names: &mut HashSet<String>,
    implemented_interfaces: &mut HashSet<String>,
) {
    let implemented_type = scope
        .get_type_index(interface_implementation.name())
        .map(|t| Type::Reference(t).as_deref(scope));
    if let Some(Type::Interface(interface_type)) = implemented_type {
        if !implemented_interfaces.insert(interface_type.identifier.clone()) {
            scope.source.print_error(
                interface_implementation.identifier.span,
                &format!(
                    "Duplicate implementation of `{}`",
                    interface_type.identifier
                ),
                &format!(
                    "{} `{}` already implements this interface",
                    get_container_type(self_type),
                    self_type.format(scope),
                ),
            );
        }
    }

    if let Some(methods) = interface_implementation.methods.as_ref() {
        for method in methods.iter() {
            check_duplicate_method(method, scope, self_type, scope_names);
        }
    }
}

fn check_duplicate_method(
    method: &FunctionNode,
    scope: &mut Scope,
    self_type: &Type,
    scope_names: &mut HashSet<String>,
) {
    if scope_names.contains(method.name()) {
        print_duplicate_member_error(scope, self_type, method);
    } else {
        let method_type = Type::Function(method.get_type(scope).clone());
        scope.add_value(method.name(), method_type);
        scope_names.insert(method.name().clone());
    }
}

fn print_duplicate_member_error(scope: &Scope, self_type: &Type, method: &FunctionNode) {
    let container_type = get_container_type(self_type);
    scope.source.print_error(
        method.signature.identifier.span,
        &format!("Duplicate {} member `{}`", container_type, method.name()),
        &format!(
            "{} `{}` already contains a {} with this name",
            container_type,
            self_type.format(scope),
            match self_type {
                Type::Enum(_) => "variant or method",
                Type::Struct(_) => "field or method",
                _ => panic!("Implementation node for non struct/enum"),
            }
        ),
    );
}

fn get_container_type(self_type: &Type) -> &'static str {
    match self_type {
        Type::Enum(_) => "enum",
        Type::Struct(_) => "struct",
        _ => panic!("Implementation node for non struct/enum"),
    }
}
