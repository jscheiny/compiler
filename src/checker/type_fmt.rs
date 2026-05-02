use std::fmt::Display;

use crate::{
    checker::Type,
    lexer::{Keyword, Symbol},
};

// TODO refactors here - maybe combine with types.rs
pub struct TypeFmt<'a> {
    pub resolved_type: &'a Type,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Array(element_type) => write!(f, "[{}]", element_type),
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
            Type::Interface(interface_type) => write!(f, "{}", interface_type.name),
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
