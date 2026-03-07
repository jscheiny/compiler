use std::{collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{Identified, StructNode},
};

pub struct StructType {
    node: Rc<StructNode>,
    pub members: HashMap<String, StructMember>,
}

impl StructType {
    pub fn from(node: Rc<StructNode>, scope: &Scope) -> Rc<StructType> {
        let mut members = HashMap::new();
        for field in node.fields.iter() {
            let member = field.get_member(scope);
            let identifier = field.id().clone();
            members.entry(identifier).or_insert(member);
        }

        if let Some(implementation) = node.implementation.as_ref() {
            for (identifier, public, function_type) in implementation.get_methods(scope) {
                members.entry(identifier).or_insert(StructMember {
                    public,
                    member_type: StructMemberType::Method(function_type),
                });
            }
        }

        Rc::new(StructType { node, members })
    }

    pub fn id(&self) -> &String {
        self.node.identifier.id()
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
