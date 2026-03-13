use std::rc::Rc;

use crate::{
    checker::{FunctionType, RuntimeType, Scope, Type},
    parser::{ExpressionNode, NameNode, Named, Node, NodeVec, TokenSpan, check_function_call},
};

pub struct AccessExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: Node<NameNode>,
    pub arguments: Option<NodeVec<ExpressionNode>>,
}

impl AccessExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        // TODO should we mutate the expected type here?
        let (scope, left_type) = self.left.check_expected(scope, expected_type);
        let function_type = left_type.to_function(&scope);
        if let Some(function_type) = function_type {
            return self.check_deferred(scope, function_type);
        }

        let field_type = get_field(&left_type, self.left.span, &self.field, &scope);
        if let Some(arguments) = self.arguments.as_ref() {
            check_function_call(scope, self.field.span, field_type, arguments)
        } else {
            (scope, field_type)
        }
    }

    pub fn check_deferred(
        &self,
        scope: Box<Scope>,
        function_type: Rc<FunctionType>,
    ) -> (Box<Scope>, Type) {
        let field_type = get_field(
            &function_type.return_type,
            self.left.span,
            &self.field,
            &scope,
        );
        let (scope, result_type) = if let Some(arguments) = self.arguments.as_ref() {
            check_function_call(scope, self.field.span, field_type, arguments)
        } else {
            (scope, field_type)
        };

        if result_type.is_error() {
            return (scope, result_type);
        }

        let deferred_type = Type::Function(Rc::new(FunctionType {
            parameters: function_type.parameters.clone(),
            return_type: Box::new(result_type),
        }));
        (scope, deferred_type)
    }
}

pub fn get_field(
    input_type: &Type,
    input_span: TokenSpan,
    field: &Node<NameNode>,
    scope: &Scope,
) -> Type {
    match input_type {
        Type::Enum(enum_type) => {
            let method = enum_type.get_method(scope, field.name());
            if let Some(method) = method {
                if !method.public {
                    check_private_access(scope, input_type, field);
                }
                Type::Function(method.function_type.clone())
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.name()),
                    &format!(
                        "enum `{}` has no such method `{}`",
                        enum_type.id(),
                        field.name()
                    ),
                );
                Type::Error
            }
        }
        Type::Function(_) => {
            scope.source.print_error(
                input_span,
                "Cannot use access operator on a function which returns another function",
                &format!("returns type: `{}`", input_type.format(scope)),
            );
            Type::Error
        }
        Type::Interface(interface_type) => {
            let method = interface_type.methods.get(field.name());
            if let Some(method) = method {
                Type::Function(method.clone())
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find method `{}`", field.name()),
                    &format!(
                        "interface `{}` has no such method `{}`",
                        interface_type.identifier,
                        field.name()
                    ),
                );
                Type::Error
            }
        }
        Type::Primitive(_) => todo!("Implement access on primitive values"),
        Type::Reference(index) => {
            let resolved_type = scope.get_type(*index).unwrap();
            get_field(&resolved_type, input_span, field, scope)
        }
        Type::Struct(struct_type) => {
            let member = struct_type.get_member(scope, field.name());
            if let Some(member) = member {
                if !member.public {
                    check_private_access(scope, input_type, field);
                }
                member.member_type.get_type()
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.name()),
                    &format!(
                        "struct `{}` has no such field or method `{}`",
                        struct_type.id(),
                        field.name()
                    ),
                );
                Type::Error
            }
        }
        Type::Tuple(_) => todo!("Implement access on tuples"),
        Type::Type(inner_type) => get_static_field(inner_type, field, scope),
        Type::Array(_) | Type::Void => {
            scope.source.print_error(
                field.span.before(),
                "Access operator is not valid for this type",
                &format!("Access on type: `{}`", input_type.format(scope)),
            );
            Type::Error
        }
        Type::Error => Type::Error,
    }
}

fn get_static_field(runtime_type: &RuntimeType, field: &Node<NameNode>, scope: &Scope) -> Type {
    match runtime_type {
        RuntimeType::Enum(enum_type) => {
            if let Some(variant_type) = enum_type.get_variant(field.name()) {
                variant_type
            } else if let Some(method) = enum_type.get_method(scope, field.name()) {
                let receiver_type = Type::Enum(enum_type.clone());
                if !method.public {
                    check_private_access(scope, &receiver_type, field);
                }
                method.function_type.clone().as_static_method(receiver_type)
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.name()),
                    &format!(
                        "enum `{}` has no such method or variant `{}`",
                        enum_type.id(),
                        field.name()
                    ),
                );
                Type::Error
            }
        }
        RuntimeType::Struct(struct_type) => {
            let member = struct_type.get_member(scope, field.name());
            if let Some(member) = member {
                let receiver_type = Type::Struct(struct_type.clone());
                if !member.public {
                    check_private_access(scope, &receiver_type, field);
                }
                member.member_type.as_static_type(receiver_type)
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.name()),
                    &format!(
                        "struct `{}` has no such field or method `{}`",
                        struct_type.id(),
                        field.name()
                    ),
                );
                Type::Error
            }
        }
    }
}

fn check_private_access(scope: &Scope, receiver_type: &Type, field: &Node<NameNode>) {
    if is_external_private_access(scope, receiver_type) {
        scope.source.print_error(
            field.span,
            &format!("Cannot access private member `{}`", field.name()),
            &format!(
                "this member is private to `{}`",
                receiver_type.format(scope),
            ),
        );
    }
}

fn is_external_private_access(scope: &Scope, receiver_type: &Type) -> bool {
    let self_type = scope.get_self_type();
    if let Some(self_type) = self_type {
        let self_type = self_type.as_deref(scope);
        if !self_type.is_equivalent_to(receiver_type, scope) {
            return true;
        }
    } else {
        return true;
    }

    return false;
}
