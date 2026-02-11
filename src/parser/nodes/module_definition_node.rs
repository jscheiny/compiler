use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{EnumNode, FunctionNode, Identified, StructNode, TypeAliasNode},
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
    pub fn check(&self, types: &mut TypeResolver, scope: &Box<Scope>) {
        match self {
            Self::Struct(node) => todo!("Struct type checking not implemented"),
            Self::Enum(node) => todo!("Enum type checking not implemented"),
            Self::Function(node) => node.check(),
            Self::TypeAlias(node) => node.check(),
        }
    }

    pub fn add_to_scope(&self, types: &mut TypeResolver, scope: &mut Scope) {
        match self {
            Self::Struct(node) => {
                scope.add_type(node.id(), Type::Struct(node.get_type(types).clone()));
                // TODO maybe add constructor value
            }
            Self::Enum(node) => {
                scope.add_type(node.id(), Type::Enum(node.get_type(types).clone()));
            }
            Self::Function(node) => {
                scope.add_value(node.id(), Type::Function(node.get_type(types).clone()));
            }
            Self::TypeAlias(node) => {
                scope.add_type(node.id(), node.get_type(types).clone());
            }
        }
    }

    pub fn resolve_type(&mut self, types: &mut TypeResolver) {
        match self {
            Self::Struct(node) => {
                let resolved_type = Type::Struct(node.get_type(types).clone());
                types.resolve(node.id(), resolved_type);
            }
            Self::Enum(node) => {
                let resolved_type = Type::Enum(node.get_type(types).clone());
                types.resolve(node.id(), resolved_type);
            }
            Self::TypeAlias(node) => {
                let resolved_type = node.get_type(types).clone();
                types.resolve(node.id(), resolved_type);
            }
            Self::Function(_) => {}
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
