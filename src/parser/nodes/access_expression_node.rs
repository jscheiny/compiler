use crate::{
    checker::{FunctionType, RuntimeType, Scope, Type, TypeResolver},
    parser::{
        ExpressionNode, Identified, IdentifierNode, Node, NodeVec, TokenSpan, check_function_call,
    },
};

pub struct AccessExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: Node<IdentifierNode>,
    pub arguments: Option<NodeVec<ExpressionNode>>,
}

impl AccessExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        // TODO should we mutate the expected type here?
        let (scope, left_type) = self.left.check_expected(scope, expected_type);
        let function_type = left_type.clone().as_function(&scope.types);
        if let Some(function_type) = function_type {
            return self.check_deferred(scope, function_type);
        }

        let field_type = get_field(&left_type, self.left.span, &self.field, &scope);
        if let Some(arguments) = self.arguments.as_ref() {
            check_function_call(scope, self.field.span, field_type, &arguments)
        } else {
            (scope, field_type)
        }
    }

    pub fn check_deferred(
        &self,
        scope: Box<Scope>,
        function_type: FunctionType,
    ) -> (Box<Scope>, Type) {
        let field_type = get_field(
            &function_type.return_type,
            self.left.span,
            &self.field,
            &scope,
        );
        let (scope, result_type) = if let Some(arguments) = self.arguments.as_ref() {
            check_function_call(scope, self.field.span, field_type, &arguments)
        } else {
            (scope, field_type)
        };

        if result_type.is_error() {
            return (scope, result_type);
        }

        let deferred_type = Type::Function(FunctionType {
            parameters: function_type.parameters,
            return_type: Box::new(result_type),
        });
        (scope, deferred_type)
    }
}

pub fn get_field(
    input_type: &Type,
    input_span: TokenSpan,
    field: &Node<IdentifierNode>,
    scope: &Scope,
) -> Type {
    match input_type {
        Type::Enum(enum_type) => {
            let method = enum_type.methods.get(field.id());
            if let Some(method) = method {
                if !method.public {
                    // TODO respect public/private access
                }
                Type::Function(method.function_type.clone())
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "enum `{}` has no such method `{}`",
                        enum_type.identifier,
                        field.id()
                    ),
                );
                Type::Error
            }
        }
        Type::Function(_) => {
            scope.source.print_error(
                input_span,
                "Cannot use access operator on a function which returns another function",
                &format!("returns type: `{}`", input_type.format(&scope.types)),
            );
            Type::Error
        }
        Type::Primitive(_) => todo!("Implement access on primitive values"),
        Type::Reference(index) => {
            let resolved_type = scope.types.get_type(*index).unwrap();
            get_field(&resolved_type, input_span, field, scope)
        }
        Type::Struct(struct_type) => {
            let member = struct_type.members.get(field.id());
            if let Some(member) = member {
                if !member.public {
                    // TODO respect public/private access
                }
                member.member_type.get_type()
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "struct `{}` has no such field or method `{}`",
                        struct_type.identifier,
                        field.id()
                    ),
                );
                Type::Error
            }
        }
        Type::Tuple(_) => todo!("Implement access on tuples"),
        Type::Type(inner_type) => get_static_field(&inner_type, field, scope),
        Type::Array(_) | Type::Void => {
            scope.source.print_error(
                field.span.previous(),
                "Access operator is not valid for this type",
                &format!("Access on type: `{}`", input_type.format(&scope.types)),
            );
            Type::Error
        }
        Type::Error => Type::Error,
    }
}

fn get_static_field(
    runtime_type: &RuntimeType,
    field: &Node<IdentifierNode>,
    scope: &Scope,
) -> Type {
    // TODO use reference types instead of expensive copies of self (or switch to RCs!)
    match runtime_type {
        RuntimeType::Enum(enum_type) => {
            if let Some(variant_type) = enum_type.get_variant(field.id()) {
                variant_type
            } else if let Some(method) = enum_type.methods.get(field.id()) {
                // TODO respect public/private access
                let self_type = get_self_type(&enum_type.identifier, &scope.types);
                method.function_type.clone().as_static_method(self_type)
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "enum `{}` has no such method or variant `{}`",
                        enum_type.identifier,
                        field.id()
                    ),
                );
                Type::Error
            }
        }
        RuntimeType::Struct(struct_type) => {
            let member = struct_type.members.get(field.id());
            if let Some(member) = member {
                // TODO respect public/private access
                let self_type = scope
                    .types
                    .get_ref(&struct_type.identifier)
                    .map(Type::Reference)
                    .unwrap_or(Type::Error);
                member.member_type.clone().as_static_type(self_type)
            } else {
                scope.source.print_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "struct `{}` has no such field or method `{}`",
                        struct_type.identifier,
                        field.id()
                    ),
                );
                Type::Error
            }
        }
    }
}

fn get_self_type(identifier: &String, types: &TypeResolver) -> Type {
    types
        .get_ref(identifier)
        .map(Type::Reference)
        .unwrap_or(Type::Error)
}
