use crate::{
    checker::{Scope, Type},
    parser::{NameNode, Node, TypeNode},
};

pub struct ClosureParameterExpressionNode {
    pub name: NameNode,
    pub parameter_type: Option<Node<TypeNode>>,
}

impl ClosureParameterExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let span = self
            .parameter_type
            .as_ref()
            .map(|node| node.span)
            .unwrap_or(self.name.span);
        scope.source.print_error(
            span,
            "unexpected type declaration",
            "type declarations should only appear in closure parameter lists",
        );

        if let Some(parameter_type) = self.parameter_type.as_ref() {
            parameter_type.get_type(&scope, None);
        }

        (scope, Type::Error)
    }
}
