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
        let mut expected_index = 0;
        for node in &self.expressions {
            if let ExpressionNode::Spread(node) = &node.value {
                // TODO pass expected type...
                let (new_scope, spread_types) = node.check_valid(scope, None);
                scope = new_scope;
                types.extend_from_slice(&spread_types);
                expected_index += spread_types.len();
            } else {
                let expected_type = expected_tuple_types.get(expected_index);
                let (new_scope, resolved_type) = node.check_expected(scope, expected_type);
                scope = new_scope;
                types.push(resolved_type);
                expected_index += 1;
            }
        }

        (scope, Type::Tuple(Rc::new(types)))
    }
}
