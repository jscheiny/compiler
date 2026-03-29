use crate::{
    checker::{Scope, Type},
    parser::{ExpressionNode, NameNode, Node, TypeNode},
};

pub struct DeclarationNode {
    pub mutable: bool,
    pub name: NameNode,
    pub type_def: Option<Node<TypeNode>>,
    pub initializer: Option<Node<ExpressionNode>>,
}

impl DeclarationNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let expected_type = self
            .type_def
            .as_ref()
            .map(|type_def| type_def.get_type(&scope, None));

        let (mut scope, resolved_type) = match self.initializer.as_ref() {
            Some(initializer) => check_initializer(scope, expected_type, initializer),
            None => (scope, Type::Error),
        };

        scope.add_value(&self.name, resolved_type);
        scope
    }
}

fn check_initializer(
    scope: Box<Scope>,
    expected_type: Option<Type>,
    initializer: &Node<ExpressionNode>,
) -> (Box<Scope>, Type) {
    let (scope, resolved_type) = initializer.check_expected(scope, expected_type.as_ref());
    let Some(expected_type) = expected_type else {
        return (scope, expected_type.unwrap_or(resolved_type));
    };

    if !resolved_type.is_assignable_to(&expected_type, &scope) {
        scope.source.print_error(
            initializer.span,
            &format!(
                "Initializer not assignable to type `{}`",
                expected_type.format(&scope)
            ),
            &format!("found type: `{}`", resolved_type.format(&scope)),
        );
    }

    (scope, expected_type)
}
