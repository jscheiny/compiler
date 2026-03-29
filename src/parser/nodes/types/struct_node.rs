use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{Scope, ScopeType, StructType},
    parser::{ImplementationNode, ImplementationType, NameNode, Node, NodeVec, StructFieldNode},
};

pub struct StructNode {
    pub name: NameNode,
    pub fields: NodeVec<StructFieldNode>,
    pub implementation: Option<Node<ImplementationNode>>,
    resolved_type: OnceCell<Rc<StructType>>,
}

impl StructNode {
    pub fn new(
        name: NameNode,
        fields: NodeVec<StructFieldNode>,
        implementation: Option<Node<ImplementationNode>>,
    ) -> Self {
        Self {
            name,
            fields,
            implementation,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(self: &Rc<Self>, scope: Box<Scope>) -> Box<Scope> {
        let index = scope.get_type_index(&self.name).unwrap();
        scope.nest(ScopeType::Struct(index), |scope| self.check_nested(scope))
    }

    fn check_nested(self: &Rc<Self>, mut scope: Box<Scope>) -> Box<Scope> {
        let mut scope_names = HashSet::new();
        for field in self.fields.iter() {
            if scope_names.insert(field.name.clone()) {
                let field_type = field.get_type(&scope).clone();
                scope.add_value(&field.name, field_type);
            } else {
                scope.source.print_error(
                    field.name.span,
                    &format!("Duplicate struct member `{}`", field.name),
                    &format!(
                        "struct `{}` already contains a field with this name",
                        self.name
                    ),
                );
            }
        }

        if let Some(implementation) = self.implementation.as_ref() {
            let self_type = ImplementationType::Struct(self.get_type());
            return implementation.check(scope, &self_type, scope_names);
        }

        scope
    }

    pub fn get_type(self: &Rc<Self>) -> Rc<StructType> {
        self.resolved_type
            .get_or_init(|| StructType::from(self.clone()))
            .clone()
    }
}
