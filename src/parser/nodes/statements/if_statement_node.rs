use crate::{
    checker::Scope,
    parser::{BlockNode, IfStatementConditionNode, Node},
};

pub struct IfStatementNode {
    pub conditions: Vec<Node<IfStatementConditionNode>>,
    pub else_branch: Option<Node<BlockNode>>,
}

impl IfStatementNode {
    pub fn check(&self, mut scope: Box<Scope>) -> Box<Scope> {
        for condition in self.conditions.iter() {
            scope = condition.check(scope);
        }

        if let Some(else_branch) = self.else_branch.as_ref() {
            (scope, _) = else_branch.check(scope, None);
        }

        scope
    }
}
