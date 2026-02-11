use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    parser::{ExpressionNode, Node, PrefixOperator},
};

pub struct PrefixOpExpressionNode {
    pub operator: Node<PrefixOperator>,
    pub expression: Box<Node<ExpressionNode>>,
}

impl PrefixOpExpressionNode {
    pub fn check(&self, _types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        match *self.operator {
            PrefixOperator::Closure => todo!("Implement type checking for prefix op Closure"),
            PrefixOperator::LogicalNot => todo!("Implement type checking for prefix op LogicalNot"),
            PrefixOperator::Negative => todo!("Implement type checking for prefix op Negative"),
            PrefixOperator::SelfRef => self.check_self_ref(scope),
        }
    }

    fn check_self_ref(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        if let ExpressionNode::Identifier(identifier) = &self.expression.value {
            let self_scope = scope.find_scope(ScopeType::Struct);
            if let Some(self_scope) = self_scope {
                let resolved_type = self_scope.lookup_local(identifier);
                if let Some(resolved_type) = resolved_type {
                    return (scope, resolved_type);
                }
                println!("Type error: cannot find value in struct or enum");
            } else {
                println!("Type error: Cannot use @ op outside of struct or enum");
            }
        } else if !matches!(&self.expression.value, ExpressionNode::Error) {
            println!("Type error: Self ref must be followed by an identifier");
        }

        (scope, Type::Error)
    }
}
