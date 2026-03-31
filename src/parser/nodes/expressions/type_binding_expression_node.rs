use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Node, NodeVec, TypeNode, bind_type},
};

pub struct TypeBindingExpressionNode {
    pub left: Box<Node<ExpressionNode>>,
    pub bound_type_parameters: NodeVec<TypeNode>,
}

impl TypeBindingExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        // TODO left expression shouldn't necessarily be a type, as in the case of a generic function with bound type args
        let (scope, unbound_type) = self.left.check_type(scope, self.left.span);
        let bound_type = bind_type(&scope, &unbound_type, &self.bound_type_parameters, None);
        (scope, bound_type)
    }
}
