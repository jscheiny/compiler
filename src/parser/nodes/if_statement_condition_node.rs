use crate::{
    checker::{Scope, ScopeType},
    parser::{BlockNode, ExpressionNode, Node, PrimitiveType},
};

pub struct IfStatementConditionNode {
    pub predicate: Node<ExpressionNode>,
    pub body: Node<BlockNode>,
}

impl IfStatementConditionNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let (scope, predicate_type) = self.predicate.check(scope);
        if !predicate_type.is_primitive(PrimitiveType::Bool, &scope.types) {
            println!(
                "Type error: If statement predicate must be of type bool, found `{}`",
                predicate_type.format(&scope.types)
            );
        }

        scope.nest(ScopeType::Block, |scope| {
            let (scope, resolved_type) = self.body.check(scope, None);
            if resolved_type.is_some() {
                println!("Type error: Unexpected body return in if block");
            }

            scope
        })
    }
}
