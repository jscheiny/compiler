use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    parser::{Node, StatementNode},
};

pub struct BlockNode {
    pub statements: Vec<Node<StatementNode>>,
}

impl BlockNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Option<Type>) {
        let mut scope = scope.derive(ScopeType::Block);
        // TODO properly type check block return types
        let mut resolved_type = None;
        for statement in self.statements.iter() {
            (scope, resolved_type) = statement.check(types, scope);
            // TODO properly handle multiple block returns
        }
        (scope.parent(), resolved_type)
    }
}
