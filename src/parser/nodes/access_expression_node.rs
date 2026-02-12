use crate::{
    checker::{Scope, StructType, Type, TypeResolver},
    parser::{ExpressionNode, Identified, IdentifierNode, Node},
};

pub struct AccessExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub field: Node<IdentifierNode>,
}

impl AccessExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, left_type) = self.left.check(types, scope);
        if let Some(struct_type) = get_struct_type(left_type, types) {
            let member = struct_type.members.get(self.field.id());
            if let Some(member) = member {
                // TODO respect public/private access
                return (scope, member.member_type.get_type());
            } else {
                println!("Type error: No field `{}` could be found", self.field.id());
            }
        }
        (scope, Type::Error)
    }
}

fn get_struct_type(input_type: Type, types: &TypeResolver) -> Option<StructType> {
    match input_type {
        Type::Enum(_) => todo!("Implement access on enums"),
        Type::Function(_) => {
            println!("Type error: No access operator on functions");
            None
        }
        Type::Primitive(_) => todo!("Implement access on primitives"),
        Type::Reference(index) => {
            let resolved_type = types.get_type(index).unwrap();
            get_struct_type(resolved_type, types)
        }
        Type::Struct(struct_type) => Some(struct_type),
        Type::Tuple(_) => todo!("Implement access on tuples"),
        Type::Error => None,
    }
}
