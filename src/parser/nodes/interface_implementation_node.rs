use std::collections::HashSet;

use crate::{
    checker::{FunctionType, Scope, Type},
    lexer::Symbol,
    parser::{FunctionNode, Identified, IdentifierNode, Node},
};

pub struct InterfaceImplementationNode {
    pub identifier: Node<IdentifierNode>,
    pub methods: Option<Vec<Node<FunctionNode>>>,
}

impl InterfaceImplementationNode {
    pub fn check(&self, mut scope: Box<Scope>, self_type: &Type) -> Box<Scope> {
        let implemented_type = scope
            .get_type_index(self.id())
            .map(|t| Type::Reference(t).as_deref(&scope));

        if let Some(implemented_type) = implemented_type.as_ref() {
            if !matches!(implemented_type, Type::Interface(_)) {
                scope.source.print_error(
                    self.identifier.span,
                    "Can only implement interfaces",
                    &format!(
                        "found non interface type: `{}`",
                        implemented_type.format(&scope)
                    ),
                );
            }
        } else {
            scope.source.print_error(
                self.identifier.span,
                &format!("Unknown type `{}`", self.identifier.id()),
                "could not find this type",
            );
        }

        let mut method_names = HashSet::new();
        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                scope = check_method(scope, method, implemented_type.as_ref());
                method_names.insert(method.id());
            }
        }

        if let Some(Type::Interface(interface_type)) = implemented_type.as_ref() {
            for (method, _) in interface_type.methods.iter() {
                if !method_names.contains(method) {
                    scope.source.print_error(
                        self.identifier.span,
                        &format!(
                            "Implementation of `{}` is incomplete",
                            interface_type.identifier
                        ),
                        &format!("does not implement method `{}`", method),
                    );
                }
            }
        }

        if self.methods.is_none() {
            match self_type {
                Type::Struct(_) => scope.source.print_error(
                    self.identifier.span.after(),
                    &format!("Cannot infer interface implementation for structs"),
                    &format!("expected `{}`", Symbol::OpenBrace),
                ),
                Type::Enum(_) => {
                    todo!("Type checking for enum default implementation of interfaces")
                }
                _ => panic!("Interface implementation node for non struct/enum"),
            }
        }

        // TODO check for empty implementation in invalid contexts
        scope
    }
}

fn check_method(
    scope: Box<Scope>,
    method: &Node<FunctionNode>,
    implemented_type: Option<&Type>,
) -> Box<Scope> {
    if let Some(Type::Interface(interface_type)) = implemented_type {
        let interface_method = interface_type.methods.get(method.id());
        if let Some(interface_method) = interface_method {
            check_method_equivalence(&scope, interface_method, method);
        }
    }

    method.check(scope)
}

fn check_method_equivalence(
    scope: &Scope,
    interface_type: &FunctionType,
    implemented_method: &Node<FunctionNode>,
) {
    let implemented_type = implemented_method.get_type(scope);
    if interface_type.parameters.len() != implemented_type.parameters.len() {
        scope.source.print_error(
            implemented_method.signature.parameters.span,
            &format!(
                "Implementation of `{}` contains {} parameters",
                implemented_method.id(),
                if implemented_type.parameters.len() > interface_type.parameters.len() {
                    "too many"
                } else {
                    "too few"
                }
            ),
            &format!(
                "Expected {} parameter{} but found {}",
                interface_type.parameters.len(),
                if interface_type.parameters.len() != 1 {
                    "s"
                } else {
                    ""
                },
                implemented_type.parameters.len()
            ),
        );
    }

    let parameters_iter = implemented_type
        .parameters
        .iter()
        .zip(interface_type.parameters.iter())
        .enumerate();
    for (index, (interface_parameter, implemented_parameter)) in parameters_iter {
        if !implemented_parameter.is_equivalent_to(interface_parameter, scope) {
            let parameter_node = &implemented_method.signature.parameters[index];
            let error_span = parameter_node
                .type_def
                .as_ref()
                .map(|t| t.span)
                .unwrap_or(parameter_node.span);
            scope.source.print_error(
                error_span,
                &format!(
                    "Parameter {} of `{}` does not match expected type from interface",
                    index + 1,
                    implemented_method.id(),
                ),
                &format!(
                    "expected type `{}`, found type: `{}`",
                    interface_parameter.format(scope),
                    implemented_parameter.format(scope)
                ),
            );
        }
    }

    if !interface_type
        .return_type
        .is_equivalent_to(&implemented_type.return_type, scope)
    {
        let error_span = implemented_method
            .signature
            .return_type
            .as_ref()
            .map(|node| node.span)
            .unwrap_or_else(|| implemented_method.body.span.start());
        scope.source.print_error(
            error_span,
            &format!(
                "Return type of `{}` does not match expected type from interface",
                implemented_method.id(),
            ),
            &format!(
                "expected type `{}`, found type: `{}`",
                interface_type.return_type.format(scope),
                implemented_type.return_type.format(scope)
            ),
        );
    }
}

impl Identified for InterfaceImplementationNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
