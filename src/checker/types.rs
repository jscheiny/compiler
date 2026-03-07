use std::rc::Rc;

use crate::{
    checker::{EnumType, FunctionType, InterfaceType, Scope, StructType, TypeFmt},
    parser::PrimitiveType,
};

// TODO reconsider this name
#[derive(Clone)]
pub enum RuntimeType {
    Enum(Rc<EnumType>),
    Struct(Rc<StructType>),
}

#[derive(Clone)]
pub enum Type {
    Array(Box<Type>),
    Enum(Rc<EnumType>),
    Function(Rc<FunctionType>),
    Interface(Rc<InterfaceType>),
    Primitive(PrimitiveType),
    Reference(usize),
    Struct(Rc<StructType>),
    Tuple(Rc<Vec<Type>>),
    Type(RuntimeType),
    Void,
    Error,
}

impl Type {
    pub fn is_assignable_to(&self, other: &Type, scope: &Scope) -> bool {
        if self.is_error() || other.is_error() {
            return true;
        }

        if let Type::Reference(_) = other {
            let resolved_other = other.deref(scope);
            return self.is_assignable_to(&resolved_other, scope);
        }

        // TODO this will need revisement as time goes on...
        match self {
            Type::Array(left) => match other {
                Type::Array(right) => left.is_assignable_to(right, scope),
                // TODO handle function type coercion better...
                _ => match self.clone().as_function(scope) {
                    Some(function_type) => {
                        Type::Function(function_type).is_assignable_to(other, scope)
                    }
                    None => false,
                },
            },
            Type::Enum(left) => match other {
                Type::Enum(right) => left.id() == right.id(),
                _ => false,
            },
            Type::Function(left) => match other {
                Type::Function(right) => {
                    left.parameters.len() == right.parameters.len()
                        && left
                            .parameters
                            .iter()
                            .zip(right.parameters.iter())
                            .all(|(left, right)| left.is_assignable_to(right, scope))
                        && left.return_type.is_assignable_to(&right.return_type, scope)
                }
                _ => false,
            },
            Type::Interface(left) => match other {
                Type::Interface(right) => left.identifier == right.identifier,
                Type::Enum(_) | Type::Struct(_) => {
                    todo!("Implement assignability for interfaces to enums/structs")
                }
                _ => false,
            },
            Type::Primitive(left) => match other {
                Type::Primitive(right) => left == right,
                _ => false,
            },
            Type::Reference(_) => self.deref(scope).is_assignable_to(other, scope),
            Type::Struct(left) => match other {
                Type::Struct(right) => left.id() == right.id(),
                _ => false,
            },
            Type::Tuple(left) => match other {
                Type::Tuple(right) => {
                    left.len() == right.len()
                        && left
                            .iter()
                            .zip(right.iter())
                            .all(|(left, right)| left.is_assignable_to(right, scope))
                }
                _ => false,
            },
            Type::Type(_) => todo!("Implement assignability for runtime types"),
            Type::Void => matches!(other, Type::Void),
            Type::Error => true,
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

    pub fn as_function(self, scope: &Scope) -> Option<Rc<FunctionType>> {
        match self.as_deref(scope) {
            Type::Array(element_type) => Some(FunctionType::new(
                Type::Primitive(PrimitiveType::Int),
                element_type.as_ref().clone(),
            )),
            Type::Function(function_type) => Some(function_type),
            Type::Type(RuntimeType::Struct(_)) => {
                todo!("Implement call operator for types (constructor)")
            }
            _ => None,
        }
    }

    pub fn as_runtime_type(self, scope: &Scope) -> Option<RuntimeType> {
        match self.as_deref(scope) {
            Type::Type(runtime_type) => Some(runtime_type),
            Type::Enum(enum_type) => Some(RuntimeType::Enum(enum_type)),
            Type::Struct(struct_type) => Some(RuntimeType::Struct(struct_type)),
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
