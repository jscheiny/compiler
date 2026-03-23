use std::rc::Rc;

use crate::{
    checker::{
        EnumType, FunctionType, GenericType, InterfaceType, Scope, StructType, TypeFmt,
        TypeParameter, TypeParameterBindings,
    },
    parser::PrimitiveType,
};

#[derive(Clone)]
pub enum Type {
    Array(Box<Type>),
    Enum(Rc<EnumType>),
    Function(Rc<FunctionType>),
    Generic(Rc<GenericType>),
    Interface(Rc<InterfaceType>),
    Primitive(PrimitiveType),
    Reference(usize),
    Struct(Rc<StructType>),
    Tuple(Rc<Vec<Type>>),
    TypeParameter(Rc<TypeParameter>),
    Void,
    Error,
}

impl Type {
    pub fn is_equivalent_to(&self, other: &Type, scope: &Scope) -> bool {
        self.is_assignable_to(other, scope) && other.is_assignable_to(self, scope)
    }

    pub fn is_assignable_to(&self, other: &Type, scope: &Scope) -> bool {
        other.is_assignable_from(self, scope)
    }

    fn is_assignable_from(&self, other: &Type, scope: &Scope) -> bool {
        if other.is_error() {
            return true;
        }

        if let Type::Reference(_) = other {
            let resolved_other = other.deref(scope);
            return self.is_assignable_from(&resolved_other, scope);
        }

        match self {
            // TODO should arrays be contravariant?
            Type::Array(left) => match other {
                Type::Array(right) => left.is_assignable_from(right, scope),
                _ => false,
            },
            Type::Enum(left) => match other {
                Type::Enum(right) => left.name() == right.name(),
                _ => false,
            },
            Type::Function(left) => match other.to_function(scope) {
                Some(right) => {
                    left.parameters.len() == right.parameters.len()
                        && left
                            .parameters
                            .iter()
                            .zip(right.parameters.iter())
                            // Parameters are contravariant
                            .all(|(left, right)| left.is_assignable_to(right, scope))
                        && left
                            .return_type
                            .is_assignable_from(&right.return_type, scope)
                }
                None => false,
            },
            Type::Generic(_) => panic!("It should not be possible to produce a generic type"),
            Type::Interface(left) => match other {
                Type::Interface(right) => left.name == right.name,
                Type::Enum(right) => right.implements(scope, left),
                Type::Struct(right) => right.implements(scope, left),
                _ => false,
            },
            Type::Primitive(left) => match other {
                Type::Primitive(right) => left == right,
                _ => false,
            },
            Type::Reference(_) => self.deref(scope).is_assignable_from(other, scope),
            Type::Struct(left) => match other {
                Type::Struct(right) => left.name() == right.name(),
                _ => false,
            },
            Type::Tuple(left) => match other {
                Type::Tuple(right) => {
                    left.len() == right.len()
                        && left
                            .iter()
                            .zip(right.iter())
                            .all(|(left, right)| left.is_assignable_from(right, scope))
                }
                _ => false,
            },
            Type::TypeParameter(left) => match other {
                Type::TypeParameter(right) => left == right,
                _ => false,
            },
            Type::Void => matches!(other, Type::Void),
            Type::Error => true,
        }
    }

    pub fn bind(&self, scope: &Scope, bindings: &TypeParameterBindings) -> Type {
        match self {
            Type::Array(t) => Type::Array(Box::new(t.bind(scope, bindings))),
            // TODO implement bind for enums
            Type::Enum(t) => Type::Enum(t.clone()),
            Type::Function(t) => Type::Function(t.bind(scope, bindings)),
            Type::Generic(_) => panic!("It should not be possible to bind a generic type"),
            // TODO implement bind for interfaces
            Type::Interface(t) => Type::Interface(t.clone()),
            Type::Primitive(t) => Type::Primitive(*t),
            Type::Reference(_) => self.deref(scope).bind(scope, bindings),
            // TODO implement bind for structs
            Type::Struct(t) => Type::Struct(t.clone()),
            Type::Tuple(types) => Type::Tuple(Rc::new(
                types.iter().map(|t| t.bind(scope, bindings)).collect(),
            )),
            Type::TypeParameter(t) => t.bind(bindings),
            Type::Void => Type::Void,
            Type::Error => Type::Error,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Type::Error)
    }

    pub fn is_primitive(&self, expected: PrimitiveType, scope: &Scope) -> bool {
        match self.deref(scope) {
            Self::Primitive(primitive) => primitive == expected,
            Self::Error => true,
            _ => false,
        }
    }

    pub fn to_function(&self, scope: &Scope) -> Option<Rc<FunctionType>> {
        match self.deref(scope) {
            Type::Array(element_type) => Some(FunctionType::simple(
                Type::Primitive(PrimitiveType::Int),
                element_type.as_ref().clone(),
            )),
            Type::Function(function_type) => Some(function_type),
            _ => None,
        }
    }

    pub fn as_deref(self, scope: &Scope) -> Type {
        match self {
            Type::Reference(index) => scope.get_type(index).unwrap_or(Type::Error).as_deref(scope),
            _ => self,
        }
    }

    pub fn deref(&self, scope: &Scope) -> Type {
        match self {
            Type::Reference(index) => scope.get_type(*index).unwrap_or(Type::Error).deref(scope),
            _ => self.clone(),
        }
    }

    pub fn format<'a>(&'a self, scope: &'a Scope) -> TypeFmt<'a> {
        TypeFmt {
            resolved_type: self,
            scope,
        }
    }
}
