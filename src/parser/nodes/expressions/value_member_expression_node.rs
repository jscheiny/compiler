use std::rc::Rc;

use crate::{
    checker::{FunctionType, MemberType, Scope, Type, Types},
    parser::{
        ExpressionNode, MethodInstanceKind, NameNode, Node, NodeVec, TokenSpan, check_function_call,
    },
};

pub struct ValueMemberExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: NameNode,
    pub arguments: Option<NodeVec<ExpressionNode>>,
}

impl ValueMemberExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        // TODO should we mutate the expected type here?
        let (scope, left_type) = self.left.check_expected(scope, expected_type);
        let function_type = left_type.to_function();
        if let Some(function_type) = function_type {
            return self.check_deferred(scope, &function_type);
        }

        let field_type = get_field(&left_type, self.left.span, &self.field, &scope);
        if let Some(arguments) = self.arguments.as_ref() {
            check_function_call(scope, self.field.span, &field_type, arguments)
        } else {
            (scope, field_type)
        }
    }

    fn check_deferred(
        &self,
        scope: Box<Scope>,
        function_type: &Rc<FunctionType>,
    ) -> (Box<Scope>, Type) {
        let field_type = get_field(
            &function_type.return_type,
            self.left.span,
            &self.field,
            &scope,
        );
        let (scope, result_type) = if let Some(arguments) = self.arguments.as_ref() {
            check_function_call(scope, self.field.span, &field_type, arguments)
        } else {
            (scope, field_type)
        };

        if result_type.is_error() {
            return (scope, result_type);
        }

        let deferred_type = Type::Function(FunctionType::new(
            function_type.parameters.clone(),
            result_type,
        ));
        (scope, deferred_type)
    }
}

pub fn get_field(
    receiver_type: &Type,
    receiver_span: TokenSpan,
    field: &NameNode,
    scope: &Scope,
) -> Type {
    match receiver_type {
        Type::Array(_) | Type::Void => {
            scope.source.print_error(
                field.span.before(),
                "Value member operator is not valid for this type",
                &format!("type: `{receiver_type}`"),
            );
            Type::Error
        }
        Type::Enum(enum_type) => {
            let method = enum_type.get_method(scope, field);
            if let Some(method) = method {
                check_member(scope, receiver_type, enum_type.name(), field, method)
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{field}`"),
                    &format!("enum `{}` has no such method `{field}`", enum_type.name()),
                );
                Type::Error
            }
        }
        Type::Function(_) => {
            scope.source.print_error(
                receiver_span,
                "Cannot use value member operator on a function which returns another function",
                &format!("returns type: `{receiver_type}`"),
            );
            Type::Error
        }
        Type::Generic(_) => panic!("It should not be possible to produce an unbound generic type"),
        Type::Interface(interface_type) => {
            let method = interface_type.methods.get(&field.value);
            if let Some(method) = method {
                Type::Function(method.function_type.clone())
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find method `{field}`"),
                    &format!(
                        "interface `{}` has no such method `{field}`",
                        interface_type.name
                    ),
                );
                Type::Error
            }
        }
        Type::Primitive(_) => todo!("Implement value member for primitive values"),
        Type::Struct(struct_type) => {
            let member = struct_type.get_member(scope, field);
            if let Some(member) = member {
                check_member(scope, receiver_type, struct_type.name(), field, member)
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{field}`"),
                    &format!(
                        "struct `{}` has no such field or method `{field}`",
                        struct_type.name(),
                    ),
                );
                Type::Error
            }
        }
        Type::Tuple(_) => todo!("Implement value member operator for tuples"),
        Type::TypeParameter(_) => todo!("Implement value member operator for type parameters"),
        Type::Error => Type::Error,
    }
}

pub fn check_member(
    scope: &Scope,
    receiver_type: &Type,
    receiver_name: &str,
    field: &NameNode,
    member: &impl MemberType,
) -> Type {
    check_private_member(scope, receiver_type, field, member);

    if member.instance_kind() == MethodInstanceKind::Static {
        scope.source.print_error(
            field.span,
            "Cannot use static member on instance value",
            &format!("`{field}` is static on type `{receiver_name}`"),
        );
    }

    member.get_type()
}

pub fn check_private_member(
    scope: &Scope,
    receiver_type: &Type,
    field: &NameNode,
    member: &impl MemberType,
) {
    if !member.is_public() && is_external_private_access(scope, receiver_type) {
        scope.source.print_error(
            field.span,
            &format!("Cannot access private member `{field}`"),
            &format!("this member is private to `{receiver_type}`"),
        );
    }
}

fn is_external_private_access(scope: &Scope, receiver_type: &Type) -> bool {
    let self_type = scope.get_self_type();
    let Some(self_type) = self_type else {
        return true;
    };

    !self_type.is_equivalent_to(receiver_type, scope)
}
