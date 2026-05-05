use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

use crate::{
    checker::{GenericType, Scope, ScopeType, Type, Types},
    parser::{NameNode, Node, TypeNode, TypeParameterListNode},
};

pub struct TypeAliasNode {
    pub name: NameNode,
    type_parameters: Option<Node<TypeParameterListNode>>,
    type_def: Node<TypeNode>,
    resolved_type: OnceCell<Type>,
    resolution_status: RefCell<ResolutionStatus>,
}

#[derive(PartialEq, Eq, Debug)]
enum ResolutionStatus {
    NotStarted,
    InProgress,
    Done,
}

impl TypeAliasNode {
    pub fn new(
        name: NameNode,
        type_parameters: Option<Node<TypeParameterListNode>>,
        type_def: Node<TypeNode>,
    ) -> Self {
        Self {
            name,
            type_parameters,
            type_def,
            resolved_type: OnceCell::new(),
            resolution_status: RefCell::new(ResolutionStatus::NotStarted),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        scope.nest_with(ScopeType::Type, |mut scope| {
            if let Some(type_parameters) = self.type_parameters.as_ref() {
                scope = type_parameters.check(scope, type_parameters.span);
            }
            let resolved_type = self.get_type(&*scope).unwrap_or(Type::Error);
            (scope, resolved_type)
        })
    }

    pub fn check_statement(&self, scope: Box<Scope>) -> Box<Scope> {
        let (mut scope, resolved_type) = self.check(scope);
        scope.add_type(&self.name, resolved_type);
        scope
    }

    pub fn get_type(&self, types: &impl Types) -> Option<Type> {
        {
            let resolution_status = self.resolution_status.borrow();
            if *resolution_status == ResolutionStatus::InProgress {
                return None;
            }
        }

        let resolved_type = self
            .resolved_type
            .get_or_init(|| {
                self.set_resolution_status(ResolutionStatus::InProgress);
                let result = self.init_type(types);
                self.set_resolution_status(ResolutionStatus::Done);
                result
            })
            .clone();
        Some(resolved_type)
    }

    fn init_type(&self, types: &impl Types) -> Type {
        let type_params = self.type_parameters.as_ref().map(|t| t.get_types_map());
        let base_type = self.type_def.get_type(types, type_params);
        let Some(type_parameters) = self.type_parameters.as_ref() else {
            return base_type;
        };

        Type::Generic(Rc::new(GenericType {
            name: self.name.clone(),
            base_type,
            type_parameters: type_parameters.get_types_list().clone(),
        }))
    }

    fn set_resolution_status(&self, status: ResolutionStatus) {
        *self.resolution_status.borrow_mut() = status;
    }
}
