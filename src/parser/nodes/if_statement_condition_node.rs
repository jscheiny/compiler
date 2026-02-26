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
            scope.source.print_type_error(
                self.predicate.span,
                "If statement predicate expected to be bool",
                &format!("found type: `{}`", predicate_type.format(&scope.types)),
            );
        }

        scope.nest(ScopeType::Block, |scope| {
            let (scope, resolved_type) = self.body.check(scope, None);
            if resolved_type.is_some() {
                // TODO we will need better infrastructure to correctly place the span for this error
                println!("Type error: Unexpected body return in if block");
            }

            scope
        })
    }
}
