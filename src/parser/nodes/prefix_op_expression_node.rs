use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node, PrefixOperator, PrimitiveType},
};

pub struct PrefixOpExpressionNode {
    pub operator: Node<PrefixOperator>,
    pub expression: Box<Node<ExpressionNode>>,
}

impl PrefixOpExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        match *self.operator {
            PrefixOperator::LogicalNot => self.check_logical_not(scope),
            PrefixOperator::Negative => self.check_negative(scope),
        }
    }

    fn check_logical_not(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, resolved_type) = self.expression.check(scope);
        if !resolved_type.is_primitive(PrimitiveType::Bool, &scope.types) {
            println!(
                "Type error: Operand of op `{:?}` should be of type bool, found `{}`",
                self.operator.value,
                resolved_type.format(&scope.types)
            );
        }

        (scope, Type::Primitive(PrimitiveType::Bool))
    }

    fn check_negative(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, resolved_type) = self.expression.check(scope);
        if resolved_type.is_primitive(PrimitiveType::Float, &scope.types)
            || resolved_type.is_primitive(PrimitiveType::Int, &scope.types)
        {
            (scope, resolved_type)
        } else {
            println!("Type error: Can only negate numeric types");
            (scope, Type::Error)
        }
    }
}
