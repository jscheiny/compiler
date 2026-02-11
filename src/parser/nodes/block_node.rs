use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{Node, StatementNode},
};

pub struct BlockNode {
    pub statements: Vec<Node<StatementNode>>,
}

impl BlockNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Option<Type>) {
        let mut scope = scope.derive();
        // TODO type check block results
        for statement in self.statements.iter() {
            let (new_scope, _resolved_type) = statement.check(types, scope);
            scope = new_scope;
        }
        (scope.parent(), None)
    }
}
