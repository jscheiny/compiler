use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    parser::{BlockNode, ExpressionNode, Node, PrimitiveType},
};

pub struct WhileLoopNode {
    pub predicate: Node<ExpressionNode>,
    pub body: Node<BlockNode>,
}

impl WhileLoopNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> Box<Scope> {
        // TODO check whether these expected bool types are actually useful?
        let bool_type = Some(&Type::Primitive(PrimitiveType::Bool));
        let (scope, predicate_type) = self.predicate.check(types, scope, bool_type);
        if !predicate_type.is_primitive(PrimitiveType::Bool, types) {
            println!("Type error: While loop predicate must be of type bool");
        }

        let scope = scope.derive(ScopeType::Loop);
        let (scope, resolved_type) = self.body.check(types, scope);
        if resolved_type.is_some() {
            println!("Type error: Unexpected body return in while loop");
        }
        scope.parent()
    }
}
