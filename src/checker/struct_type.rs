use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{Identified, StructNode},
};

pub struct StructType {
    node: Rc<StructNode>,
    members: OnceCell<HashMap<String, StructMember>>,
}

impl StructType {
    pub fn from(node: Rc<StructNode>) -> Rc<StructType> {
        Rc::new(StructType {
            node,
            members: OnceCell::new(),
        })
    }

    pub fn id(&self) -> &String {
        self.node.identifier.id()
    }

    pub fn get_member(&self, scope: &Scope, identifier: &String) -> Option<&StructMember> {
        self.members
            .get_or_init(|| self.init_members(scope))
            .get(identifier)
    }

    fn init_members(&self, scope: &Scope) -> HashMap<String, StructMember> {
        let scope = scope.global();
        let mut members = HashMap::new();
        for field in self.node.fields.iter() {
            let member = field.get_member(scope);
            let identifier = field.id().clone();
            members.entry(identifier).or_insert(member);
        }

        if let Some(implementation) = self.node.implementation.as_ref() {
            for (identifier, public, function_type) in implementation.get_methods(scope) {
                members.entry(identifier).or_insert(StructMember {
                    public,
                    member_type: StructMemberType::Method(function_type),
                });
            }
        }

        members
    }
}

pub struct StructMember {
    pub public: bool,
    pub member_type: StructMemberType,
}

pub enum StructMemberType {
    Field(Type),
    Method(Rc<FunctionType>),
}

impl StructMemberType {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Field(field_type) => field_type.clone(),
            Self::Method(function_type) => Type::Function(function_type.clone()),
        }
    }

    pub fn as_static_type(&self, self_type: Type) -> Type {
        match self {
            Self::Field(field_type) => {
                Type::Function(FunctionType::new(self_type, field_type.clone()))
            }
            Self::Method(function_type) => function_type.clone().as_static_method(self_type),
        }
    }
}
