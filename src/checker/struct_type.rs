use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, InterfaceType, Scope, Type},
    parser::{Named, StructNode},
};

pub struct StructType {
    node: Rc<StructNode>,
    constructor: OnceCell<Rc<FunctionType>>,
    members: OnceCell<HashMap<String, StructMember>>,
}

impl StructType {
    pub fn from(node: Rc<StructNode>) -> Rc<StructType> {
        Rc::new(StructType {
            node,
            constructor: OnceCell::new(),
            members: OnceCell::new(),
        })
    }

    pub fn name(&self) -> &String {
        &self.node.name
    }

    pub fn get_constructor(self: &Rc<Self>, scope: &Scope) -> Rc<FunctionType> {
        self.constructor
            .get_or_init(|| self.init_constructor(scope))
            .clone()
    }

    fn init_constructor(self: &Rc<Self>, scope: &Scope) -> Rc<FunctionType> {
        let scope = scope.global();
        let parameters = self
            .node
            .fields
            .iter()
            .map(|field| field.get_type(scope).clone())
            .collect();
        let return_type = Type::Struct(self.clone());

        Rc::new(FunctionType {
            parameters,
            return_type: Box::new(return_type),
        })
    }

    pub fn get_member(&self, scope: &Scope, name: &String) -> Option<&StructMember> {
        self.members
            .get_or_init(|| self.init_members(scope))
            .get(name)
    }

    fn init_members(&self, scope: &Scope) -> HashMap<String, StructMember> {
        let scope = scope.global();
        let mut members = HashMap::new();
        for field in self.node.fields.iter() {
            let member = field.get_member(scope);
            let name = field.name().clone();
            members.entry(name).or_insert(member);
        }

        if let Some(implementation) = self.node.implementation.as_ref() {
            for (name, public, function_type) in implementation.get_methods(scope) {
                members.entry(name).or_insert(StructMember {
                    public,
                    member_type: StructMemberType::Method(function_type),
                });
            }
        }

        members
    }

    pub fn implements(&self, scope: &Scope, interface_type: &Rc<InterfaceType>) -> bool {
        self.node
            .implementation
            .as_ref()
            .map(|implementation| implementation.implements(scope, interface_type))
            .unwrap_or(false)
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
