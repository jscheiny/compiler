use crate::{
    checker::{Scope, Type, TypeResolver},
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
        Type::Enum(_) => todo!("Implement access on enums"),
        Type::Function(_) => {
            println!("Type error: No access operator on functions");
            None
        }
        Type::Primitive(_) => todo!("Implement access on primitives"),
        Type::Reference(index) => {
            let resolved_type = types.get_type(index).unwrap();
            get_field(resolved_type, field, types)
        }
        Type::Struct(struct_type) => {
            let member = struct_type.members.get(field);
            if let Some(member) = member {
                // TODO respect public/private access
                Some(member.member_type.get_type())
            } else {
                println!("Type error: No field `{}` could be found", field);
                None
            }
        }
        Type::Tuple(_) => todo!("Implement access on tuples"),
        Type::Error => None,
    }
}
