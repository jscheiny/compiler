use crate::{
    checker::{Scope, TypeResolver},
    parser::{ExpressionNode, Identified, IdentifierNode, Node, TypeNode},
};

pub struct DeclarationNode {
    pub mutable: bool,
    pub identifier: Node<IdentifierNode>,
    pub type_def: Option<Node<TypeNode>>,
    pub initializer: Option<Node<ExpressionNode>>,
}

impl DeclarationNode {
    pub fn check(&self, types: &TypeResolver, mut scope: Box<Scope>) -> Box<Scope> {
        let resolved_type = match self.type_def.as_ref() {
            Some(type_def) => type_def.get_type(types),
            None => todo!("Declaration type inference not implemented"),
        };
        if let Some(initializer) = self.initializer.as_ref() {
            let (new_scope, _resolved_type) = initializer.check(types, scope);
            // TODO handle type checking
            scope = new_scope
        }
        scope.add(self.id(), resolved_type);
        scope
    }
}

impl Identified for DeclarationNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
