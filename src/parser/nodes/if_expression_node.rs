use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, PrimitiveType},
};

pub struct IfExpressionNode {
    pub predicate: Box<Node<ExpressionNode>>,
    pub if_true: Box<Node<ExpressionNode>>,
    pub if_false: Box<Node<ExpressionNode>>,
}

impl IfExpressionNode {
    pub fn check(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        let (scope, predicate_type) = self.predicate.check(types, scope);
        if !predicate_type.is_primitive(PrimitiveType::Bool, types) {
            println!("Type error: If expression predicate must be of type bool");
        }

        let (scope, true_type) = self.if_true.check_expected(types, scope, expected_type);
        let expected_type = expected_type.or(Some(&true_type));
        let (scope, false_type) = self.if_false.check_expected(types, scope, expected_type);

        if true_type.is_assignable_to(&false_type, types) {
            (scope, false_type)
        } else if false_type.is_assignable_to(&true_type, types) {
            (scope, true_type)
        } else {
            println!(
                "Type error: Types of branches of if expression do not match: `{}` and `{}`",
                true_type.format(types),
                false_type.format(types)
            );
            (scope, true_type)
        }
    }
}
