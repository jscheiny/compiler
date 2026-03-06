use std::collections::HashSet;

use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{Identified, ImplementationEntryNode, MethodNode, Node},
};

pub struct ImplementationNode {
    pub entries: Vec<Node<ImplementationEntryNode>>,
}

#[derive(Clone, Copy)]
pub enum ImplementationNodeType {
    Struct,
    Enum,
}

impl ImplementationNode {
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
    pub fn get_methods(&self, scope: &Scope) -> Vec<(String, bool, FunctionType)> {
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
                    println!("impl {}", implementation.id());
                    println!("{:?}", scope.get_type_index(implementation.id()));
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
}

fn check_method_duplicate(
    method: &MethodNode,
    scope: &mut Scope,
    implementation_type: ImplementationNodeType,
    container_name: &str,
    scope_names: &mut HashSet<String>,
) {
    if scope_names.contains(method.id()) {
        print_duplicate_member_error(&scope, implementation_type, container_name, method);
    } else {
        let method_type = Type::Function(method.function.get_type(&scope).clone());
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
