use crate::{
    checker::{Scope, TypeResolver},
    parser::{BlockNode, IfStatementConditionNode, Node},
};

pub struct IfStatementNode {
    pub conditions: Vec<Node<IfStatementConditionNode>>,
    pub else_branch: Option<Node<BlockNode>>,
}

impl IfStatementNode {
    pub fn check(&self, types: &TypeResolver, mut scope: Box<Scope>) -> Box<Scope> {
        for condition in self.conditions.iter() {
            scope = condition.check(types, scope);
        }

        if let Some(else_branch) = self.else_branch.as_ref() {
            let (new_scope, resolved_type) = else_branch.check(types, scope, None);
            if resolved_type.is_some() {
                println!("Type error: Unexpected body return in else block");
            }
            scope = new_scope;
        }

        scope
    }
}
