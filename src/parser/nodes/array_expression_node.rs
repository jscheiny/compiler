use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node},
};

pub struct ArrayExpressionNode {
    pub elements: Vec<Node<ExpressionNode>>,
}

impl ArrayExpressionNode {
    pub fn check(&self, mut scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let expected_element_type = match expected_type {
            Some(Type::Array(t)) => Some(t.as_ref()),
            _ => None,
        };

        let mut resolved_type = None;

        for node in self.elements.iter() {
            let (new_scope, element_type) = node.check_expected(scope, expected_element_type);
            scope = new_scope;

            if let Some(t) = resolved_type.as_ref() {
                if element_type.is_assignable_to(t, &scope.types) {
                    // Element type matches no error and keep going
                } else if t.is_assignable_to(&element_type, &scope.types) {
                    resolved_type = Some(element_type);
                } else {
                    println!(
                        "Type error: Mismatching types in array literal `{}` and `{}`",
                        t.format(&scope.types),
                        element_type.format(&scope.types)
                    );
                }
            } else {
                resolved_type = Some(element_type);
            }
        }

        if let Some(resolved_type) = resolved_type {
            (scope, Type::Array(Box::new(resolved_type)))
        } else if let Some(expected_element_type) = expected_element_type {
            (scope, Type::Array(Box::new(expected_element_type.clone())))
        } else {
            println!("Type error: Could not infer type of empty array");
            (scope, Type::Array(Box::new(Type::Error)))
        }
    }
}
