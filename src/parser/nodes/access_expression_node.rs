use crate::{
    checker::{FunctionType, RuntimeType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, Identified, IdentifierNode, Node, TokenSpan},
};

pub struct AccessExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: Node<IdentifierNode>,
}

impl AccessExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        // TODO should we change the expected type here?
        let (scope, left_type) = self.left.check_expected(scope, expected_type);
        let field_type = get_field(&left_type, &self.field, &scope);
        (scope, field_type.unwrap_or(Type::Error))
    }
}

pub fn get_field(input_type: &Type, field: &Node<IdentifierNode>, scope: &Scope) -> Option<Type> {
    match input_type {
        Type::Enum(enum_type) => {
            let method = enum_type.methods.get(field.id());
            if let Some(method) = method {
                if !method.public {
                    // TODO respect public/private access
                }
                Some(Type::Function(method.function_type.clone()))
            } else {
                scope.source.print_type_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "enum `{}` has no such method `{}`",
                        enum_type.identifier,
                        field.id()
                    ),
                );
                None
            }
        }
        Type::Function(function_type) => {
            let result_type = get_field(&function_type.return_type, field, scope);
            result_type.map(|result_type| {
                Type::Function(FunctionType {
                    parameters: function_type.parameters.clone(),
                    return_type: Box::new(result_type),
                })
            })
        }
        Type::Primitive(_) => todo!("Implement access on primitive values"),
        Type::Reference(index) => {
            let resolved_type = scope.types.get_type(*index).unwrap();
            get_field(&resolved_type, field, scope)
        }
        Type::Struct(struct_type) => {
            let member = struct_type.members.get(field.id());
            if let Some(member) = member {
                if !member.public {
                    // TODO respect public/private access
                }
                Some(member.member_type.get_type())
            } else {
                scope.source.print_type_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "struct `{}` has no such field or method `{}`",
                        struct_type.identifier,
                        field.id()
                    ),
                );
                None
            }
        }
        Type::Tuple(_) => todo!("Implement access on tuples"),
        Type::Type(inner_type) => get_static_field(&inner_type, field, scope),
        Type::Array(_) | Type::Void => {
            let span = TokenSpan::singleton_of(field.span.start_index - 1);
            scope.source.print_type_error(
                span,
                "Access operator is not valid for this type",
                &format!("Access on type: `{}`", input_type.format(&scope.types)),
            );
            None
        }
        Type::Error => None,
    }
}

fn get_static_field(
    runtime_type: &RuntimeType,
    field: &Node<IdentifierNode>,
    scope: &Scope,
) -> Option<Type> {
    // TODO use reference types instead of expensive copies of self (or switch to RCs!)
    match runtime_type {
        RuntimeType::Enum(enum_type) => {
            if let Some(variant_type) = enum_type.get_variant(field.id()) {
                Some(variant_type)
            } else if let Some(method) = enum_type.methods.get(field.id()) {
                // TODO respect public/private access
                let self_type = get_self_type(&enum_type.identifier, &scope.types);
                Some(method.function_type.clone().as_static_method(self_type))
            } else {
                scope.source.print_type_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "enum `{}` has no such method or variant `{}`",
                        enum_type.identifier,
                        field.id()
                    ),
                );
                None
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
                Some(member.member_type.clone().as_static_type(self_type))
            } else {
                scope.source.print_type_error(
                    field.span,
                    &format!("Could not find field `{}`", field.id()),
                    &format!(
                        "struct `{}` has no such field or method `{}`",
                        struct_type.identifier,
                        field.id()
                    ),
                );
                None
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
