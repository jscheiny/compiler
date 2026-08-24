use std::{collections::HashSet, rc::Rc};

use crate::{
    checker::{EnumType, MethodType, Scope, Type, Types},
    lexer::{Symbol, Token},
    parser::{ImplementationType, MethodInstanceKind, MethodNode, NameNode, Node},
};

pub struct InterfaceImplementationNode {
    pub name: NameNode,
    pub methods: Option<Vec<Node<MethodNode>>>,
}

impl InterfaceImplementationNode {
    pub fn check(&self, mut scope: Box<Scope>, self_type: &ImplementationType) -> Box<Scope> {
        let implemented_type = scope.get_type(&self.name);

        if let Some(implemented_type) = implemented_type.as_ref() {
            if !matches!(implemented_type, Type::Interface(_)) {
                scope.source.print_error(
                    self.name.span,
                    "Can only implement interfaces",
                    &format!("found non interface type: `{implemented_type}`"),
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
                scope = check_method(scope, &method, implemented_type.as_ref());
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
            for (variant_name, variant_type) in enum_type.get_variants(scope) {
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
    method: &Node<MethodNode>,
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
    interface_method: &MethodType,
    implemented_method: &Node<MethodNode>,
) {
    check_method_instance_kind(scope, interface_method, implemented_method);

    let implemented_type = implemented_method.function.get_type(scope);
    let expected_parameters = interface_method.function_type.parameters.len();
    let actual_parameters = implemented_type.parameters.len();
    if expected_parameters != actual_parameters {
        scope.source.print_error(
            implemented_method.function.signature.body_parameters.span,
            &format!(
                "Implementation of `{}` contains {} parameters",
                implemented_method.name(),
                if actual_parameters > expected_parameters {
                    "too many"
                } else {
                    "too few"
                }
            ),
            &format!(
                "Expected {} parameter{} but found {}",
                expected_parameters,
                if expected_parameters == 1 { "" } else { "s" },
                actual_parameters
            ),
        );
    }

    let parameters_iter = implemented_type
        .parameters
        .iter()
        .zip(interface_method.function_type.parameters.iter())
        .enumerate();
    for (index, (interface_parameter, implemented_parameter)) in parameters_iter {
        if !implemented_parameter.is_equivalent_to(interface_parameter, scope) {
            let body_parameter_index = implemented_method
                .function
                .signature
                .get_body_parameter_index(scope, index);
            let parameter_node =
                &implemented_method.function.signature.body_parameters[body_parameter_index];
            let error_span = parameter_node
                .type_def
                .as_ref()
                .map_or(parameter_node.span, |t| t.span);
            // TODO this error span could be better for spread parameters
            scope.source.print_error(
                error_span,
                &format!(
                    "Parameter {} of `{}` does not match expected type from interface",
                    index + 1,
                    implemented_method.name(),
                ),
                &format!(
                    "expected type `{implemented_parameter}`, found type: `{interface_parameter}`",
                ),
            );
        }
    }

    if !interface_method
        .function_type
        .return_type
        .is_equivalent_to(&implemented_type.return_type, scope)
    {
        let error_span = implemented_method
            .function
            .signature
            .return_type
            .as_ref()
            .map_or_else(
                || implemented_method.function.body.span.start(),
                |node| node.span,
            );
        scope.source.print_error(
            error_span,
            &format!(
                "Return type of `{}` does not match expected type from interface",
                implemented_method.name(),
            ),
            &format!(
                "expected type `{}`, found type: `{}`",
                interface_method.function_type.return_type, implemented_type.return_type
            ),
        );
    }
}

fn check_method_instance_kind(
    scope: &Scope,
    interface_method: &MethodType,
    implemented_method: &Node<MethodNode>,
) {
    if interface_method.instance_kind == implemented_method.instance_kind {
        return;
    }

    let mut span = implemented_method.function.signature.name.span.before();
    let span_token = scope.source.get_token(span.start_index);
    // If the user forgot to use a instance kind specifier, mark the error at the method name
    if !matches!(span_token, Token::Symbol(Symbol::Dot | Symbol::DoubleColon)) {
        span = implemented_method.function.signature.name.span;
    }

    scope.source.print_error(
        span,
        &format!(
            "Method `{}` is {}static here which does not match the interface definition",
            implemented_method.name(),
            match implemented_method.instance_kind {
                MethodInstanceKind::Instance => "not ",
                MethodInstanceKind::Static => "",
            },
        ),
        &format!(
            "expected `{}`",
            match interface_method.instance_kind {
                MethodInstanceKind::Instance => Symbol::Dot,
                MethodInstanceKind::Static => Symbol::DoubleColon,
            }
        ),
    );
}
