use crate::{
    checker::{EnumType, FunctionType, StructType, TypeFmt, TypeResolver},
    parser::PrimitiveType,
};

// TODO reconsider this name
#[derive(Clone, Debug)]
pub enum RuntimeType {
    Enum(EnumType),
    Struct(StructType),
}

#[derive(Clone, Debug)]
pub enum Type {
    Array(Box<Type>),
    Enum(EnumType),
    Function(FunctionType),
    Primitive(PrimitiveType),
    Reference(usize),
    Struct(StructType),
    Tuple(Vec<Type>),
    Type(RuntimeType),
    Void,
    Error,
}

impl Type {
    pub fn is_assignable_to(&self, other: &Type, types: &TypeResolver) -> bool {
        if matches!(self, Type::Error) || matches!(other, Type::Error) {
            return true;
        }
        if let Type::Reference(_) = other {
            let resolved_other = other.deref(types);
            return self.is_assignable_to(&resolved_other, types);
        }

        // TODO this will need revisement as time goes on...
        match self {
            Type::Array(left) => match other {
                Type::Array(right) => left.is_assignable_to(right, types),
                // TODO handle function type coercion better...
                _ => match self.clone().as_function(types) {
                    Some(function_type) => {
                        Type::Function(function_type).is_assignable_to(other, types)
                    }
                    None => false,
                },
            },
            Type::Enum(left) => match other {
                Type::Enum(right) => left.identifier == right.identifier,
                _ => false,
            },
            Type::Function(left) => match other {
                Type::Function(right) => {
                    left.parameters.len() == right.parameters.len()
                        && left
                            .parameters
                            .iter()
                            .zip(right.parameters.iter())
                            .all(|(left, right)| left.is_assignable_to(right, types))
                        && left.return_type.is_assignable_to(&right.return_type, types)
                }
                _ => false,
            },
            Type::Primitive(left) => match other {
                Type::Primitive(right) => left == right,
                _ => false,
            },
            Type::Reference(_) => self.deref(types).is_assignable_to(other, types),
            Type::Struct(left) => match other {
                Type::Struct(right) => left.identifier == right.identifier,
                _ => false,
            },
            Type::Tuple(left) => match other {
                Type::Tuple(right) => {
                    left.len() == right.len()
                        && left
                            .iter()
                            .zip(right)
                            .all(|(left, right)| left.is_assignable_to(right, types))
                }
                _ => false,
            },
            Type::Type(_) => todo!("Implement assignability for runtime types"),
            Type::Void => matches!(other, Type::Void),
            Type::Error => true,
        }
    }

    pub fn is_primitive(&self, expected: PrimitiveType, types: &TypeResolver) -> bool {
        match self.deref(types) {
            Self::Primitive(primitive) => primitive == expected,
            Self::Error => true,
            _ => false,
        }
    }

    pub fn as_function(self, types: &TypeResolver) -> Option<FunctionType> {
        match self.as_deref(types) {
            Type::Array(element_type) => Some(FunctionType::new(
                Type::Primitive(PrimitiveType::Int),
                element_type.as_ref().clone(),
            )),
            Type::Function(function_type) => Some(function_type.clone()),
            Type::Type(RuntimeType::Struct(_)) => {
                todo!("Implement call operator for types (constructor)")
            }
            _ => None,
        }
    }

    pub fn as_runtime_type(self, types: &TypeResolver) -> Option<RuntimeType> {
        match self.as_deref(types) {
            Type::Type(runtime_type) => Some(runtime_type),
            Type::Enum(enum_type) => Some(RuntimeType::Enum(enum_type)),
            Type::Struct(struct_type) => Some(RuntimeType::Struct(struct_type)),
            _ => None,
        }
    }

    pub fn as_deref(self, types: &TypeResolver) -> Type {
        match self {
            Type::Reference(index) => types.get_type(index).unwrap_or(Type::Error).as_deref(types),
            _ => self,
        }
    }

    pub fn deref(&self, types: &TypeResolver) -> Type {
        match self {
            Type::Reference(index) => types.get_type(*index).unwrap_or(Type::Error).deref(types),
            _ => self.clone(),
        }
    }

    pub fn format<'a>(&'a self, types: &'a TypeResolver) -> TypeFmt<'a> {
        TypeFmt {
            resolved_type: self,
            types,
        }
    }
}
