use std::ffi::os_str::Display;

use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{ExpressionNode, Identified, IdentifierNode, Node, TypeNode},
};

pub struct DeclarationNode {
    pub mutable: bool,
    pub identifier: Node<IdentifierNode>,
    pub type_def: Option<Node<TypeNode>>,
    pub initializer: Option<Node<ExpressionNode>>,
}

impl DeclarationNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> Box<Scope> {
        let expected_type = self
            .type_def
            .as_ref()
            .map(|type_def| type_def.get_type(types));

        let (mut scope, resolved_type) = match self.initializer.as_ref() {
            Some(initializer) => {
                let (new_scope, resolved_type) = initializer.check(types, scope);
                if let Some(expected_type) = expected_type.as_ref() {
                    if !resolved_type.is_assignable_to(expected_type, types) {
                        println!(
                            "Type error: Could not assign expression of type `{}` to variable of type `{}`",
                            resolved_type.format(types),
                            expected_type.format(types)
                        );
                    }
                }
                // TODO check that resolved type matches expected type
                (new_scope, expected_type.unwrap_or(resolved_type))
            }
            None => (scope, Type::Error),
        };

        scope.add(self.id(), resolved_type);
        scope
    }
}

impl Identified for DeclarationNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
