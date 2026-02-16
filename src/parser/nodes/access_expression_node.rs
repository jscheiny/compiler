use crate::{
    checker::{RuntimeType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, Identified, IdentifierNode, Node},
};

pub struct AccessExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: Node<IdentifierNode>,
}

impl AccessExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, left_type) = self.left.check(types, scope);
        let field_type = get_field(left_type, self.field.id(), types);
        (scope, field_type.unwrap_or(Type::Error))
    }
}

fn get_field(input_type: Type, field: &String, types: &TypeResolver) -> Option<Type> {
    match input_type {
        Type::Enum(_) => todo!("Implement access on enum values"),
        Type::Function(_) => {
            println!("Type error: No access operator on functions");
            None
        }
        Type::Primitive(_) => todo!("Implement access on primitive values"),
        Type::Reference(index) => {
            let resolved_type = types.get_type(index).unwrap();
            get_field(resolved_type, field, types)
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
    match runtime_type {
        RuntimeType::Struct(struct_type) => {
            let member = struct_type.members.get(field);
            if let Some(member) = member {
                // TODO respect public/private access
                let self_type = types
                    .get_ref(&struct_type.identifier)
                    .map(Type::Reference)
                    .unwrap_or(Type::Error);
                Some(member.member_type.get_static_type(self_type))
            } else {
                println!(
                    "Type error: No field `{}` of type `{}` could be found",
                    field, struct_type.identifier
                );
                None
            }
        }
    }
}
