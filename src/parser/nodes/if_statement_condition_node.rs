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
            scope.source.print_error(
                self.predicate.span,
                "If statement predicate expected to be bool",
                &format!("found type: `{}`", predicate_type.format(&scope.types)),
            );
        }

        scope.nest(ScopeType::Block, |scope| {
            let (scope, _) = self.body.check(scope, None);
            scope
        })
    }
}
