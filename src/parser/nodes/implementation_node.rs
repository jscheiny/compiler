use std::collections::HashSet;

use crate::{
    checker::{Scope, Type},
    parser::{Identified, InterfaceImplementationNode, MethodNode, Node},
};

// TODO this should probably be a single vector of enums, so that we can process things in order...
pub struct ImplementationNode {
    pub methods: Vec<Node<MethodNode>>,
    pub interface_implementations: Vec<Node<InterfaceImplementationNode>>,
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
        for method in self.methods.iter() {
            if scope_names.contains(method.id()) {
                print_duplicate_member_error(&scope, implementation_type, container_name, method);
            } else {
                let method_type = Type::Function(method.function.get_type(&scope).clone());
                scope.add_value(method.id(), method_type);
                scope_names.insert(method.id().clone());
            }
        }

        for method in self.methods.iter() {
            scope = method.check(scope)
        }

        scope
    }
}

fn print_duplicate_member_error(
    scope: &Scope,
    implementation_type: ImplementationNodeType,
    container_name: &str,
    method: &Node<MethodNode>,
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
