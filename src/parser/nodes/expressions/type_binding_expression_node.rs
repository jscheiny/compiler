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
        let ExpressionNode::Name(name) = &self.left.value else {
            let (scope, left_type) = self.left.check(scope);
            // TODO this should not error all the time.
            // We should check for generic function types when those are implemented
            scope.source.print_error(
                self.left.span,
                "Cannot bind to a non-generic type",
                &format!("type `{}` is not generic", left_type.format(&scope)),
            );
            return (scope, Type::Error);
        };

        let Some(type_index) = scope.get_type_index(name) else {
            scope.source.print_error(
                self.left.span,
                &format!("Unknown type `{name}`"),
                "could not find a type with this name",
            );
            return (scope, Type::Error);
        };

        let unbound_type = Type::Reference(type_index).deref(&scope);
        let bound_type = bind_type(&scope, &unbound_type, &self.bound_type_parameters, None);
        (scope, bound_type)
    }
}
