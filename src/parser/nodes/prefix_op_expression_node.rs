use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, PrefixOperator, PrimitiveType},
};

pub struct PrefixOpExpressionNode {
    pub operator: Node<PrefixOperator>,
    pub expression: Box<Node<ExpressionNode>>,
}

impl PrefixOpExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        match *self.operator {
            PrefixOperator::Closure => todo!("Implement type checking for prefix op Closure"),
            PrefixOperator::LogicalNot => self.check_logical_not(types, scope),
            PrefixOperator::Negative => self.check_negative(types, scope),
        }
    }

    fn check_logical_not(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let bool_type = Some(&Type::Primitive(PrimitiveType::Bool));
        let (scope, resolved_type) = self.expression.check(types, scope, bool_type);
        if !resolved_type.is_primitive(PrimitiveType::Bool, types) {
            println!(
                "Type error: Operand of op `{:?}` should be of type bool",
                self.operator.value
            );
        }

        (scope, Type::Primitive(PrimitiveType::Bool))
    }

    fn check_negative(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, resolved_type) = self.expression.check(types, scope, None);
        if resolved_type.is_primitive(PrimitiveType::Float, types)
            || resolved_type.is_primitive(PrimitiveType::Int, types)
        {
            (scope, resolved_type)
        } else {
            println!("Type error: Can only negate numeric types");
            (scope, Type::Error)
        }
    }
}
