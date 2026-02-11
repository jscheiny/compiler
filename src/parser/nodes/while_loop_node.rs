use crate::{
    checker::{Scope, ScopeType, TypeResolver},
    parser::{BlockNode, ExpressionNode, Node, PrimitiveType},
};

pub struct WhileLoopNode {
    pub predicate: Node<ExpressionNode>,
    pub body: Node<BlockNode>,
}

impl WhileLoopNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> Box<Scope> {
        let (scope, predicate_type) = self.predicate.check(types, scope);
        if !predicate_type.is_primitive(PrimitiveType::Bool) {
            println!("Type error: While loop predicate must be bool");
        }

        let scope = scope.derive(ScopeType::Loop);
        let (scope, resolved_type) = self.body.check(types, scope);
        if resolved_type.is_some() {
            println!("Type error: Unexpected body return in while loop");
        }
        scope.parent()
    }
}
