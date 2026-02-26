use crate::{
    checker::{RuntimeType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, Identified, IdentifierNode, Node},
};

pub struct AccessExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: Node<IdentifierNode>,
}

impl AccessExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, left_type) = self.left.check(scope);
        let field_type = get_field(&left_type, self.field.id(), &scope.types);
        (scope, field_type.unwrap_or(Type::Error))
    }
}

pub fn get_field(input_type: &Type, field: &String, types: &TypeResolver) -> Option<Type> {
    match input_type {
        Type::Array(_) => {
            // TODO consider if we should allow this
            println!("Type error: No access operator on arrays");
            None
        }
        Type::Enum(enum_type) => {
            let method = enum_type.methods.get(field);
            if let Some(method) = method {
                if !method.public {
                    // TODO respect public/private access
                    println!("Type error maybe? Check privacy here");
                }
                Some(Type::Function(method.function_type.clone()))
            } else {
                println!(
                    "Type error: No field `{}` of type `{}` could be found",
                    field, enum_type.identifier,
                );
                None
            }
        }
        Type::Function(_) => {
            // TODO add access operator based on return type
            println!("Type error: No access operator on functions");
            None
        }
        Type::Primitive(_) => todo!("Implement access on primitive values"),
        Type::Reference(index) => {
            let resolved_type = types.get_type(*index).unwrap();
            get_field(&resolved_type, field, types)
        }
        Type::Struct(struct_type) => {
            let member = struct_type.members.get(field);
            if let Some(member) = member {
                if !member.public {
                    // TODO respect public/private access
                    println!("Type error maybe? Check privacy here");
                }
                Some(member.member_type.get_type())
            } else {
                println!(
                    "Type error: No field `{}` of type `{}` could be found",
                    field, struct_type.identifier
                );
                None
            }
        }
        Type::Tuple(_) => todo!("Implement access on tuples"),
        Type::Type(inner_type) => get_static_field(&inner_type, field, types),
        Type::Void => {
            println!("Type error: No access operator on void");
            None
        }
        Type::Error => None,
    }
}

fn get_static_field(
    runtime_type: &RuntimeType,
    field: &String,
    types: &TypeResolver,
) -> Option<Type> {
    // TODO use reference types instead of expensive copies of self (or switch to RCs!)
    match runtime_type {
        RuntimeType::Enum(enum_type) => {
            if let Some(variant_type) = enum_type.get_variant(field) {
                Some(variant_type)
            } else if let Some(method) = enum_type.methods.get(field) {
                // TODO respect public/private access
                let self_type = get_self_type(&enum_type.identifier, types);
                Some(method.function_type.clone().as_static_method(self_type))
            } else {
                println!(
                    "Type error: No method or variant `{}` of enum `{}` could be found",
                    field, enum_type.identifier
                );
                None
            }
        }
        RuntimeType::Struct(struct_type) => {
            let member = struct_type.members.get(field);
            if let Some(member) = member {
                // TODO respect public/private access
                let self_type = types
                    .get_ref(&struct_type.identifier)
                    .map(Type::Reference)
                    .unwrap_or(Type::Error);
                Some(member.member_type.clone().as_static_type(self_type))
            } else {
                println!(
                    "Type error: No field `{}` of struct `{}` could be found",
                    field, struct_type.identifier
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
