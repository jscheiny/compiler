use std::{fmt::Display, rc::Rc};

use crate::{
    checker::{
        EnumType, FunctionType, GenericType, InterfaceType, Scope, StructType, TypeParameter,
        TypeParameterBindings, Types,
    },
    lexer::{Keyword, Symbol},
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
            Type::Function(left) => match other.to_function() {
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
                Type::Interface(right) => left.name() == right.name(),
                Type::Enum(right) => right.implements(scope, left),
                Type::Struct(right) => right.implements(scope, left),
                _ => false,
            },
            Type::Primitive(left) => match other {
                Type::Primitive(right) => left == right,
                _ => false,
            },
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

    pub fn bind(&self, types: &impl Types, bindings: &TypeParameterBindings) -> Type {
        match self {
            Type::Array(t) => Type::Array(Box::new(t.bind(types, bindings))),
            // TODO implement bind for enums
            Type::Enum(t) => Type::Enum(t.clone()),
            Type::Function(t) => Type::Function(t.bind(types, bindings)),
            Type::Generic(_) => panic!("It should not be possible to bind a generic type"),
            // TODO implement bind for interfaces
            Type::Interface(t) => Type::Interface(t.clone()),
            Type::Primitive(t) => Type::Primitive(*t),
            // TODO implement bind for structs
            Type::Struct(t) => Type::Struct(t.clone()),
            Type::Tuple(elements) => Type::Tuple(Rc::new(
                elements.iter().map(|t| t.bind(types, bindings)).collect(),
            )),
            Type::TypeParameter(t) => t.bind(bindings),
            Type::Void => Type::Void,
            Type::Error => Type::Error,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Type::Error)
    }

    pub fn is_primitive(&self, expected: PrimitiveType) -> bool {
        match self {
            Self::Primitive(primitive) => *primitive == expected,
            Self::Error => true,
            _ => false,
        }
    }

    pub fn to_function(&self) -> Option<Rc<FunctionType>> {
        match self {
            Type::Array(element_type) => Some(FunctionType::simple(
                Type::Primitive(PrimitiveType::Int),
                element_type.as_ref().clone(),
            )),
            Type::Function(function_type) => Some(function_type.clone()),
            _ => None,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Array(element_type) => write!(f, "[{element_type}]"),
            Type::Enum(enum_type) => write!(f, "{}", enum_type.name()),
            Type::Function(function_type) => {
                let show_parentheses = function_type.parameters.len() != 1
                    || matches!(function_type.parameters[0], Type::Tuple(_));
                if show_parentheses {
                    write!(f, "(")?;
                }
                write_list(f, &function_type.parameters)?;
                if show_parentheses {
                    write!(f, ")")?;
                }
                write!(f, " {} {}", Symbol::ThickArrow, function_type.return_type)
            }
            Type::Generic(generic_type) => {
                write!(f, "{}[", generic_type.name)?;
                write_list(f, &generic_type.type_parameters)?;
                write!(f, "]")
            }
            Type::Interface(interface_type) => write!(f, "{}", interface_type.name()),
            Type::Primitive(primitive_type) => write!(f, "{primitive_type}"),
            Type::Struct(struct_type) => write!(f, "{}", struct_type.name()),
            Type::Tuple(items) => {
                write!(f, "(")?;
                write_list(f, items)?;
                write!(f, ")")
            }
            Type::TypeParameter(type_parameter) => write!(f, "{}", type_parameter.name),
            Type::Void => write!(f, "{}", Keyword::Void),
            Type::Error => write!(f, "{{Unknown}}"),
        }
    }
}

fn write_list<T: Display>(f: &mut std::fmt::Formatter<'_>, list: &[T]) -> std::fmt::Result {
    for (index, element) in list.iter().enumerate() {
        write!(f, "{element}")?;
        if index != list.len() - 1 {
            write!(f, ", ")?;
        }
    }

    Ok(())
}
