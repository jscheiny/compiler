use std::rc::Rc;

use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node},
};

pub struct TupleExpressionNode {
    pub expressions: Vec<Node<ExpressionNode>>,
}

impl TupleExpressionNode {
    pub fn check(&self, mut scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let expected_tuple_types = if let Some(Type::Tuple(types)) = expected_type {
            types
        } else {
            &vec![]
        };

        let mut types = vec![];
        for (index, node) in self.expressions.iter().enumerate() {
            let expected_type = expected_tuple_types.get(index);
            let (new_scope, resolved_type) = node.check_expected(scope, expected_type);
            scope = new_scope;
            types.push(resolved_type);
        }

        (scope, Type::Tuple(Rc::new(types)))
    }
}
