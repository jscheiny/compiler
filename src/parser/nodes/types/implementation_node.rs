use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{EnumType, FunctionType, InterfaceType, Scope, StructType, Type, Types},
    parser::{FunctionNode, ImplementationEntryNode, InterfaceImplementationNode, Node},
};

pub struct ImplementationNode {
    pub entries: Vec<Node<ImplementationEntryNode>>,
    implemented_interfaces: OnceCell<HashSet<usize>>,
}

pub enum ImplementationType {
    Enum(Rc<EnumType>),
    Struct(Rc<StructType>),
}

impl ImplementationType {
    pub fn as_type(&self) -> Type {
        match self {
            Self::Enum(enum_type) => Type::Enum(enum_type.clone()),
            Self::Struct(struct_type) => Type::Struct(struct_type.clone()),
        }
    }
}

pub struct Method {
    pub public: bool,
    pub name: String,
    pub function_type: Rc<FunctionType>,
}

impl ImplementationNode {
    pub fn new(entries: Vec<Node<ImplementationEntryNode>>) -> Self {
        Self {
            entries,
            implemented_interfaces: OnceCell::new(),
        }
    }

    pub fn check(
        &self,
        mut scope: Box<Scope>,
        self_type: &ImplementationType,
        mut scope_names: HashSet<String>,
    ) -> Box<Scope> {
        let mut implemented_interfaces = HashSet::new();
        for entry in &self.entries {
            match &entry.value {
                ImplementationEntryNode::Interface(interface) => check_duplicate_interface(
                    interface,
                    &mut scope,
                    self_type,
                    &mut scope_names,
                    &mut implemented_interfaces,
                ),
                ImplementationEntryNode::Method(method) => check_duplicate_method(
                    &method.function,
                    &mut scope,
                    self_type,
                    &mut scope_names,
                ),
            }
        }

        for entry in &self.entries {
            scope = entry.check(scope, self_type);
        }

        scope
    }

    pub fn get_methods(&self, scope: &Scope) -> Vec<Method> {
        let mut methods = vec![];
        for entry in &self.entries {
            match &entry.value {
                ImplementationEntryNode::Method(method) => {
                    methods.push(Method {
                        public: method.public,
                        name: method.function.name().clone(),
                        function_type: method.function.get_type(scope).clone(),
                    });
                }
                ImplementationEntryNode::Interface(implementation) => {
                    let interface_type = scope.get_type(&implementation.name);
                    if let Some(Type::Interface(interface_type)) = interface_type {
                        for (name, function_type) in &interface_type.methods {
                            methods.push(Method {
                                public: true,
                                name: name.clone(),
                                function_type: function_type.clone(),
                            });
                        }
                    }
                }
            }
        }

        methods
    }

    pub fn implements(&self, scope: &Scope, interface_type: &Rc<InterfaceType>) -> bool {
        let interface_type_id = scope.global().get_type_id(&interface_type.name);
        match interface_type_id {
            Some(type_id) => self
                .implemented_interfaces
                .get_or_init(|| self.init_implemented_interfaces(scope))
                .contains(&type_id),
            None => false,
        }
    }

    fn init_implemented_interfaces(&self, scope: &Scope) -> HashSet<usize> {
        let mut result = HashSet::new();
        for entry in &self.entries {
            if let ImplementationEntryNode::Interface(node) = &entry.value {
                let type_id = scope.get_type_id(&node.name);
                if let Some(type_id) = type_id {
                    result.insert(type_id);
                }
            }
        }

        result
    }
}

fn check_duplicate_interface(
    interface_implementation: &InterfaceImplementationNode,
    scope: &mut Scope,
    self_type: &ImplementationType,
    scope_names: &mut HashSet<String>,
    implemented_interfaces: &mut HashSet<String>,
) {
    // TODO Should this use type ids instead?
    let implemented_type = scope.get_type(&interface_implementation.name);
    if let Some(Type::Interface(interface_type)) = implemented_type
        && !implemented_interfaces.insert(interface_type.name.clone())
    {
        scope.source.print_error(
            interface_implementation.name.span,
            &format!("Duplicate implementation of `{}`", interface_type.name),
            &format!(
                "{} `{}` already implements this interface",
                get_container_type(self_type),
                self_type.as_type(),
            ),
        );
    }

    if let Some(methods) = interface_implementation.methods.as_ref() {
        for method in methods {
            check_duplicate_method(method, scope, self_type, scope_names);
        }
    }
}

fn check_duplicate_method(
    method: &FunctionNode,
    scope: &mut Scope,
    self_type: &ImplementationType,
    scope_names: &mut HashSet<String>,
) {
    if scope_names.contains(method.name()) {
        print_duplicate_member_error(scope, self_type, method);
    } else {
        let method_type = Type::Function(method.get_type(scope).clone());
        scope.add_value(method.name(), method_type);
        scope_names.insert(method.name().clone());
    }
}

fn print_duplicate_member_error(
    scope: &Scope,
    self_type: &ImplementationType,
    method: &FunctionNode,
) {
    let container_type = get_container_type(self_type);
    scope.source.print_error(
        method.signature.name.span,
        &format!("Duplicate {} member `{}`", container_type, method.name()),
        &format!(
            "{} `{}` already contains a {} with this name",
            container_type,
            self_type.as_type(),
            match self_type {
                ImplementationType::Enum(_) => "variant or method",
                ImplementationType::Struct(_) => "field or method",
            }
        ),
    );
}

fn get_container_type(self_type: &ImplementationType) -> &'static str {
    match self_type {
        ImplementationType::Enum(_) => "enum",
        ImplementationType::Struct(_) => "struct",
    }
}
