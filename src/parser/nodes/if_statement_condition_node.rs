use crate::{
    checker::{Scope, ScopeType, TypeResolver},
    parser::{BlockNode, ExpressionNode, Node, PrimitiveType},
};

pub struct IfStatementConditionNode {
    pub predicate: Node<ExpressionNode>,
    pub body: Node<BlockNode>,
}

impl IfStatementConditionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> Box<Scope> {
        let (scope, predicate_type) = self.predicate.check(types, scope);
        if !predicate_type.is_primitive(PrimitiveType::Bool) {
            println!("Type error: If statement predicate must be of type bool");
        }

        let scope = scope.derive(ScopeType::Block);
        let (scope, resolved_type) = self.body.check(types, scope);
        if resolved_type.is_some() {
            println!("Type error: Unexpected body return in if block");
        }

        scope.parent()
    }
}
