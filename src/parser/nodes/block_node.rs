use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    parser::{Node, StatementNode},
};

pub struct BlockNode {
    pub statements: Vec<Node<StatementNode>>,
}

impl BlockNode {
    pub fn check(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Option<Type>) {
        let mut scope = scope.derive(ScopeType::Block);
        // TODO error if no block return statement when one might be expected
        let mut resolved_type = None;
        for statement in self.statements.iter() {
            let (new_scope, statement_type) = statement.check(types, scope, expected_type);
            scope = new_scope;
            // Resolve type to the type of the first block return, everything after is effectively dead code.
            if resolved_type.is_none() {
                resolved_type = statement_type;
            }
        }
        (scope.parent(), resolved_type)
    }
}
