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
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, predicate_type) = self.predicate.check(types, scope);
        if !predicate_type.is_primitive(PrimitiveType::Bool) {
            println!("Type error: If expression predicate must be of type bool");
        }

        let (scope, true_type) = self.if_true.check(types, scope);
        let (scope, false_type) = self.if_false.check(types, scope);
        // TODO check if true type and false types match

        if !matches!(true_type, Type::Error) {
            (scope, true_type)
        } else {
            (scope, false_type)
        }
    }
}
