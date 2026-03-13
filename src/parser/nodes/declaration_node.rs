use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, NameNode, Named, Node, TypeNode},
};

pub struct DeclarationNode {
    pub mutable: bool,
    pub name: Node<NameNode>,
    pub type_def: Option<Node<TypeNode>>,
    pub initializer: Option<Node<ExpressionNode>>,
}

impl DeclarationNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let expected_type = self
            .type_def
            .as_ref()
            .map(|type_def| type_def.get_type(&scope));

        let (mut scope, resolved_type) = match self.initializer.as_ref() {
            Some(initializer) => {
                let (new_scope, resolved_type) =
                    initializer.check_expected(scope, expected_type.as_ref());
                if let Some(expected_type) = expected_type.as_ref() {
                    if !resolved_type.is_assignable_to(expected_type, &new_scope) {
                        new_scope.source.print_error(
                            initializer.span,
                            &format!(
                                "Initializer not assignable to type `{}`",
                                expected_type.format(&new_scope)
                            ),
                            &format!("found type: `{}`", resolved_type.format(&new_scope)),
                        );
                    }
                }
                (new_scope, expected_type.unwrap_or(resolved_type))
            }
            None => (scope, Type::Error),
        };

        scope.add_value(self.name(), resolved_type);
        scope
    }
}

impl Named for DeclarationNode {
    fn name(&self) -> &String {
        self.name.name()
    }
}
