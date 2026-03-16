use std::fmt::Display;

use crate::{
    checker::{RuntimeType, Scope, Type},
    lexer::{Keyword, Symbol},
};

pub struct TypeFmt<'a> {
    pub resolved_type: &'a Type,
    pub scope: &'a Scope,
}

// TODO function for lists of items
impl Display for TypeFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.resolved_type {
            Type::Array(element_type) => write!(f, "[{}]", element_type.format(self.scope)),
            Type::Enum(enum_type) => write!(f, "{}", enum_type.name()),
            Type::Function(function_type) => {
                if function_type.parameters.len() != 1 {
                    write!(f, "(")?;
                }
                for (index, parameter) in function_type.parameters.iter().enumerate() {
                    write!(f, "{}", parameter.format(self.scope))?;
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
                    function_type.return_type.format(self.scope)
                )
            }
            Type::Generic(generic_type) => {
                write!(f, "{}[", generic_type.name)?;
                for (index, type_param) in generic_type.parameter_list.iter().enumerate() {
                    write!(f, "{}", type_param.name)?;
                    if index != generic_type.parameter_list.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Type::Interface(interface_type) => write!(f, "{}", interface_type.name),
            Type::Primitive(primitive_type) => write!(f, "{}", primitive_type),
            Type::Reference(index) => {
                write!(
                    f,
                    "{}",
                    self.scope
                        .get_type(*index)
                        .unwrap_or(Type::Error)
                        .format(self.scope)
                )
            }
            Type::Struct(struct_type) => write!(f, "{}", struct_type.name()),
            Type::Tuple(items) => {
                write!(f, "(")?;
                for (index, item) in items.iter().enumerate() {
                    write!(f, "{}", item.format(self.scope))?;
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
                    RuntimeType::Enum(enum_type) => enum_type.name(),
                    RuntimeType::Struct(struct_type) => struct_type.name(),
                }
            ),
            Type::TypeParameter(type_parameter) => write!(f, "{}", type_parameter.name),
            Type::Void => write!(f, "{}", Keyword::Void),
            Type::Error => write!(f, "<error-type>"),
        }
    }
}
