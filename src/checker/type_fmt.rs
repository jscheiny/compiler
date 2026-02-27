use std::fmt::Display;

use crate::{
    checker::{RuntimeType, Type, TypeResolver},
    lexer::{Keyword, Symbol},
};

pub struct TypeFmt<'a> {
    pub resolved_type: &'a Type,
    pub types: &'a TypeResolver,
}

impl Display for TypeFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.resolved_type {
            Type::Array(element_type) => write!(f, "[{}]", element_type.format(self.types)),
            Type::Enum(enum_type) => write!(f, "{}", enum_type.identifier),
            Type::Function(function_type) => {
                if function_type.parameters.len() != 1 {
                    write!(f, "(")?;
                }
                for (index, parameter) in function_type.parameters.iter().enumerate() {
                    write!(f, "{}", parameter.format(self.types))?;
                    if index != function_type.parameters.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                if function_type.parameters.len() != 1 {
                    write!(f, ")")?;
                }
                write!(
                    f,
                    " {} {}",
                    Symbol::ThickArrow,
                    function_type.return_type.format(self.types)
                )
            }
            Type::Primitive(primitive_type) => write!(f, "{}", primitive_type),
            Type::Reference(index) => {
                write!(
                    f,
                    "{}",
                    self.types
                        .get_type(*index)
                        .unwrap_or(Type::Error)
                        .format(self.types)
                )
            }
            Type::Struct(struct_type) => write!(f, "{}", struct_type.identifier),
            Type::Tuple(items) => {
                write!(f, "(")?;
                for (index, item) in items.iter().enumerate() {
                    write!(f, "{}", item.format(self.types))?;
                    if index != items.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Type::Type(runtime_type) => write!(
                f,
                "Runtime({})",
                match runtime_type {
                    RuntimeType::Enum(enum_type) => &enum_type.identifier,
                    RuntimeType::Struct(struct_type) => &struct_type.identifier,
                }
            ),
            Type::Void => write!(f, "{}", Keyword::Void),
            Type::Error => write!(f, "<error-type>"),
        }
    }
}
