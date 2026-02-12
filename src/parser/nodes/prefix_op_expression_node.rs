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
            PrefixOperator::Negative => todo!("Implement type checking for prefix op Negative"),
        }
    }

    fn check_logical_not(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, resolved_type) = self.expression.check(types, scope);
        if !resolved_type.is_primitive(PrimitiveType::Bool) {
            println!(
                "Type error: Operand of op `{:?}` should be of type bool",
                self.operator.value
            );
        }

        (scope, Type::Primitive(PrimitiveType::Bool))
    }
}
