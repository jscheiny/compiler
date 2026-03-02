use crate::{
    checker::{RuntimeType, Scope, Type},
    parser::{EnumNode, FunctionNode, Identified, IdentifierNode, Node, StructNode, TypeAliasNode},
};

pub struct ExportableModuleDefinitionNode {
    pub public: bool,
    pub definition: ModuleDefinitionNode,
}

pub enum ModuleDefinitionNode {
    Struct(StructNode),
    Enum(EnumNode),
    Function(FunctionNode),
    TypeAlias(TypeAliasNode),
}

impl ModuleDefinitionNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        match self {
            Self::Struct(node) => node.check(scope),
            Self::Enum(node) => node.check(scope),
            Self::Function(node) => node.check(scope),
            Self::TypeAlias(node) => {
                node.check();
                scope
            }
        }
    }

    pub fn add_to_scope(&self, scope: &mut Scope) {
        match self {
            Self::Enum(node) => {
                let enum_type = RuntimeType::Enum(node.get_type(scope).clone());
                scope.add_value(node.id(), Type::Type(enum_type));
            }
            Self::Struct(node) => {
                let struct_type = RuntimeType::Struct(node.get_type(scope).clone());
                scope.add_value(node.id(), Type::Type(struct_type));
            }
            Self::Function(node) => {
                scope.add_value(node.id(), Type::Function(node.get_type(scope).clone()));
            }
            // TODO Consider how these are added to scope
            Self::TypeAlias(_) => {}
        }
    }

    pub fn resolve_type(&mut self, scope: &mut Scope) {
        let resolved_type = match self {
            Self::Struct(node) => Some(Type::Struct(node.get_type(scope).clone())),
            Self::Enum(node) => Some(Type::Enum(node.get_type(scope).clone())),
            Self::TypeAlias(node) => Some(node.get_type(scope).clone()),
            Self::Function(_) => None,
        };

        if let Some(resolved_type) = resolved_type {
            // TODO can we do this without an unwrap?
            scope
                .types
                .as_mut()
                .unwrap()
                .resolve(self.id(), resolved_type);
        }
    }

    pub fn identifier(&self) -> &Node<IdentifierNode> {
        match self {
            Self::Struct(node) => &node.identifier,
            Self::Enum(node) => &node.identifier,
            Self::Function(node) => &node.identifier,
            Self::TypeAlias(node) => &node.identifier,
        }
    }
}

impl Identified for ModuleDefinitionNode {
    fn id(&self) -> &String {
        match self {
            Self::Struct(node) => node.id(),
            Self::Enum(node) => node.id(),
            Self::TypeAlias(node) => node.id(),
            Self::Function(node) => node.id(),
        }
    }
}
