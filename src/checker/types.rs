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

        if let Type::Reference(other_ref) = other {
            let resolved_other = types.get_type(*other_ref).unwrap_or(Type::Error);
            return self.is_assignable_to(&resolved_other, types);
        }

        // TODO this will need revisement as time goes on...
        match self {
            Type::Enum(left) => match other {
                Type::Enum(right) => left.identifier == right.identifier,
                _ => false,
            },
            Type::Function(left) => match other.as_function(types) {
                Some(right) => {
                    left.parameters.len() == right.parameters.len()
                        && left
                            .parameters
                            .iter()
                            .zip(right.parameters)
                            .all(|(left, right)| left.is_assignable_to(&right, types))
                        && left.return_type.is_assignable_to(&right.return_type, types)
                }
                None => false,
            },
            Type::Primitive(left) => match other {
                Type::Primitive(right) => left == right,
                _ => false,
            },
            Type::Reference(index) => types
                .get_type(*index)
                .unwrap_or(Type::Error)
                .is_assignable_to(other, types),
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
        match self {
            Self::Primitive(primitive) => *primitive == expected,
            Self::Reference(index) => types
                .get_type(*index)
                .unwrap_or(Type::Error)
                .is_primitive(expected, types),
            Self::Error => true,
            _ => false,
        }
    }

    pub fn as_function(&self, types: &TypeResolver) -> Option<FunctionType> {
        match self {
            Type::Function(function_type) => Some(function_type.clone()),
            Type::Reference(index) => types
                .get_type(*index)
                .unwrap_or(Type::Error)
                .as_function(types),
            Type::Type(_) => todo!("Implement call operator for types (constructor)"),
            _ => None,
        }
    }

    pub fn format<'a>(&'a self, types: &'a TypeResolver) -> TypeFmt<'a> {
        TypeFmt {
            resolved_type: self,
            types,
        }
    }
}
