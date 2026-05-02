use std::{collections::HashSet, rc::Rc};

use crate::{
    checker::{EnumType, FunctionType, Scope, Type, Types},
    lexer::Symbol,
    parser::{FunctionNode, ImplementationType, NameNode, Node},
};

pub struct InterfaceImplementationNode {
    pub name: NameNode,
    pub methods: Option<Vec<Node<FunctionNode>>>,
}

impl InterfaceImplementationNode {
    pub fn check(&self, mut scope: Box<Scope>, self_type: &ImplementationType) -> Box<Scope> {
        let implemented_type = scope.get_type(&self.name);

        if let Some(implemented_type) = implemented_type.as_ref() {
            if !matches!(implemented_type, Type::Interface(_)) {
                scope.source.print_error(
                    self.name.span,
                    "Can only implement interfaces",
                    &format!("found non interface type: `{}`", implemented_type),
                );
            }
        } else {
            scope.source.print_error(
                self.name.span,
                &format!("Unknown type `{}`", self.name),
                "could not find this type",
            );
        }

        let mut method_names = HashSet::new();
        if let Some(methods) = self.methods.as_ref() {
            for method in methods {
                scope = check_method(scope, method, implemented_type.as_ref());
                method_names.insert(method.name());
            }

            if let Some(Type::Interface(interface_type)) = implemented_type.as_ref() {
                for method in interface_type.methods.keys() {
                    if !method_names.contains(method) {
                        scope.source.print_error(
                            self.name.span,
                            &format!("Implementation of `{}` is incomplete", interface_type.name),
                            &format!("does not implement method `{method}`"),
                        );
                    }
                }
            }
        }

        if self.methods.is_none() {
            match self_type {
                ImplementationType::Struct(_) => scope.source.print_error(
                    self.name.span.after(),
                    "Cannot infer interface implementation for structs",
                    &format!("expected `{}`", Symbol::OpenBrace),
                ),
                ImplementationType::Enum(enum_type) => self.check_enum_default_implementation(
                    &scope,
                    enum_type,
                    implemented_type.as_ref(),
                ),
            }
        }

        scope
    }

    fn check_enum_default_implementation(
        &self,
        scope: &Scope,
        enum_type: &Rc<EnumType>,
        implemented_type: Option<&Type>,
    ) {
        if let Some(Type::Interface(interface_type)) = implemented_type {
            for (variant_name, variant_type) in &enum_type.variants {
                if let Some(variant_type) = variant_type {
                    let implements_interface = match variant_type.clone() {
                        Type::Enum(e) => e.implements(scope, interface_type),
                        Type::Struct(s) => s.implements(scope, interface_type),
                        _ => false,
                    };
                    if !implements_interface {
                        scope.source.print_error(
                            self.name.span,
                            "Cannot infer interface implementation",
                            &format!(
                                "variant `{}` does not implement `{}`",
                                variant_name, interface_type.name
                            ),
                        );
                    }
                } else {
                    scope.source.print_error(
                        self.name.span,
                        "Cannot infer interface implementation",
                        &format!("variant `{variant_name}` is untyped"),
                    );
                }
            }
        }
    }
}

fn check_method(
    scope: Box<Scope>,
    method: &Node<FunctionNode>,
    implemented_type: Option<&Type>,
) -> Box<Scope> {
    if let Some(Type::Interface(interface_type)) = implemented_type {
        let interface_method = interface_type.methods.get(method.name());
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
                implemented_method.name(),
                if implemented_type.parameters.len() > interface_type.parameters.len() {
                    "too many"
                } else {
                    "too few"
                }
            ),
            &format!(
                "Expected {} parameter{} but found {}",
                interface_type.parameters.len(),
                if interface_type.parameters.len() == 1 {
                    ""
                } else {
                    "s"
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
                .map_or(parameter_node.span, |t| t.span);
            scope.source.print_error(
                error_span,
                &format!(
                    "Parameter {} of `{}` does not match expected type from interface",
                    index + 1,
                    implemented_method.name(),
                ),
                &format!(
                    "expected type `{}`, found type: `{}`",
                    interface_parameter, implemented_parameter
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
            .map_or_else(|| implemented_method.body.span.start(), |node| node.span);
        scope.source.print_error(
            error_span,
            &format!(
                "Return type of `{}` does not match expected type from interface",
                implemented_method.name(),
            ),
            &format!(
                "expected type `{}`, found type: `{}`",
                interface_type.return_type, implemented_type.return_type
            ),
        );
    }
}
