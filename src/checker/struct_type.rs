use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, InterfaceType, Scope, Type, Types},
    parser::StructNode,
};

pub struct StructType {
    node: Rc<StructNode>,
    constructor: OnceCell<Rc<FunctionType>>,
    members: OnceCell<HashMap<String, StructMember>>,
}

impl StructType {
    pub fn from(node: Rc<StructNode>, types: &impl Types) -> Rc<StructType> {
        let struct_type = Rc::new(StructType {
            node,
            constructor: OnceCell::new(),
            members: OnceCell::new(),
        });
        // Immediately initialize constructor using the module level types
        struct_type.get_constructor(types);
        struct_type
    }

    pub fn name(&self) -> &String {
        &self.node.name
    }

    pub fn get_constructor(self: &Rc<Self>, types: &impl Types) -> Rc<FunctionType> {
        self.constructor
            .get_or_init(|| self.init_constructor(types))
            .clone()
    }

    fn init_constructor(self: &Rc<Self>, types: &impl Types) -> Rc<FunctionType> {
        let parameters = self
            .node
            .fields
            .iter()
            .map(|field| field.get_type(types).clone())
            .collect();
        let return_type = Type::Struct(self.clone());

        FunctionType::new(parameters, return_type)
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
            let name = field.name.clone();
            members.entry(name).or_insert(member);
        }

        if let Some(implementation) = self.node.implementation.as_ref() {
            for method in implementation.get_methods(scope) {
                members.entry(method.name).or_insert(StructMember {
                    public: method.public,
                    member_type: StructMemberType::Method(method.function_type),
                });
            }
        }

        members
    }

    pub fn implements(&self, scope: &Scope, interface_type: &Rc<InterfaceType>) -> bool {
        self.node
            .implementation
            .as_ref()
            .is_some_and(|implementation| implementation.implements(scope, interface_type))
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
                Type::Function(FunctionType::simple(self_type, field_type.clone()))
            }
            Self::Method(function_type) => function_type.clone().as_static_method(self_type),
        }
    }
}
