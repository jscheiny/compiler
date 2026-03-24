use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, NameNode, Node, check_private_member},
};

pub struct MemberTypeExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: NameNode,
}

impl MemberTypeExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        if let ExpressionNode::TypeBinding(binding) = &self.left.value {
            let (scope, receiver_type) = binding.check(scope);
            let resolved_type = self.get_static_field(&scope, &receiver_type);
            return (scope, resolved_type);
        }

        let ExpressionNode::Name(name) = &self.left.value else {
            let (scope, _) = self.left.check(scope);
            scope.source.print_error(
                self.left.span,
                "Cannot use type member operator on an expression",
                "must be a type",
            );
            return (scope, Type::Error);
        };

        let Some(type_index) = scope.get_type_index(name) else {
            scope.source.print_error(
                self.left.span,
                &format!("Unknown type `{}`", name),
                "could not find a type with this name",
            );
            return (scope, Type::Error);
        };

        let receiver_type = Type::Reference(type_index).deref(&scope);
        let resolved_type = self.get_static_field(&scope, &receiver_type);
        (scope, resolved_type)
    }

    fn get_static_field(&self, scope: &Scope, receiver_type: &Type) -> Type {
        match receiver_type.deref(scope) {
            Type::Enum(enum_type) => {
                if let Some(variant_type) = enum_type.get_variant(&self.field.value) {
                    variant_type
                } else if let Some(method) = enum_type.get_method(scope, &self.field) {
                    let receiver_type = Type::Enum(enum_type.clone());
                    if !method.public {
                        check_private_member(scope, &receiver_type, &self.field);
                    }
                    method.function_type.clone().as_static_method(receiver_type)
                } else {
                    scope.source.print_error(
                        self.field.span,
                        &format!("Could not find field `{}`", self.field),
                        &format!(
                            "enum `{}` has no such method or variant `{}`",
                            enum_type.name(),
                            self.field
                        ),
                    );
                    Type::Error
                }
            }
            Type::Struct(struct_type) => {
                let member = struct_type.get_member(scope, &self.field.value);
                if let Some(member) = member {
                    let receiver_type = Type::Struct(struct_type.clone());
                    if !member.public {
                        check_private_member(scope, &receiver_type, &self.field);
                    }
                    member.member_type.as_static_type(receiver_type)
                } else {
                    scope.source.print_error(
                        self.field.span,
                        &format!("Could not find field `{}`", self.field),
                        &format!(
                            "struct `{}` has no such field or method `{}`",
                            struct_type.name(),
                            self.field
                        ),
                    );
                    Type::Error
                }
            }
            Type::Generic(_) => todo!("Implement member type operator for generic types"),
            Type::Error => Type::Error,
            _ => {
                scope.source.print_error(
                    self.left.span,
                    "Cannot access properties of this type",
                    &format!(
                        "type `{}` does not have accessible properties",
                        receiver_type.format(scope)
                    ),
                );
                Type::Error
            }
        }
    }
}
