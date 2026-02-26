use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, Identified, IdentifierNode, Node, TypeNode},
};

pub struct DeclarationNode {
    pub mutable: bool,
    pub identifier: Node<IdentifierNode>,
    pub type_def: Option<Node<TypeNode>>,
    pub initializer: Option<Node<ExpressionNode>>,
}

impl DeclarationNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let expected_type = self
            .type_def
            .as_ref()
            .map(|type_def| type_def.get_type(&scope.types, &scope.source));

        let (mut scope, resolved_type) = match self.initializer.as_ref() {
            Some(initializer) => {
                let (new_scope, resolved_type) =
                    initializer.check_expected(scope, expected_type.as_ref());
                if let Some(expected_type) = expected_type.as_ref() {
                    if !resolved_type.is_assignable_to(expected_type, &new_scope.types) {
                        new_scope.source.print_type_error(
                            initializer.span,
                            &format!(
                                "Initializer not assignable to type `{}`",
                                expected_type.format(&new_scope.types)
                            ),
                            &format!("found type: `{}`", resolved_type.format(&new_scope.types)),
                        );
                    }
                }
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
