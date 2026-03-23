use std::fmt::Display;

use crate::{
    checker::{Scope, Type},
    lexer::{Keyword, Symbol},
};

pub struct TypeFmt<'a> {
    pub resolved_type: &'a Type,
    pub scope: &'a Scope,
}

impl Display for TypeFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.resolved_type {
            Type::Array(element_type) => write!(f, "[{}]", element_type.format(self.scope)),
            Type::Enum(enum_type) => write!(f, "{}", enum_type.name()),
            Type::Function(function_type) => {
                if function_type.parameters.len() != 1 {
                    write!(f, "(")?;
                }
                self.write_types_list(f, &function_type.parameters)?;
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
                write_slice(f, &generic_type.type_parameters)?;
                write!(f, "]")
            }
            Type::Interface(interface_type) => write!(f, "{}", interface_type.name),
            Type::Primitive(primitive_type) => write!(f, "{}", primitive_type),
            Type::Reference(_) => {
                let resolved_type = self.resolved_type.deref(self.scope);
                write!(f, "{}", resolved_type.format(self.scope))
            }
            Type::Struct(struct_type) => write!(f, "{}", struct_type.name()),
            Type::Tuple(items) => {
                write!(f, "(")?;
                self.write_types_list(f, items)?;
                write!(f, ")")
            }
            Type::TypeParameter(type_parameter) => write!(f, "{}", type_parameter.name),
            Type::Void => write!(f, "{}", Keyword::Void),
            Type::Error => write!(f, "{{Unknown}}"),
        }
    }
}

impl<'a> TypeFmt<'a> {
    fn write_types_list(&self, f: &mut std::fmt::Formatter<'_>, list: &[Type]) -> std::fmt::Result {
        let mut iter = list.iter().map(|t| t.format(self.scope));
        write_iter(f, &mut iter, list.len())
    }
}

fn write_slice<T: Display>(f: &mut std::fmt::Formatter<'_>, list: &[T]) -> std::fmt::Result {
    write_iter(f, &mut list.iter(), list.len())
}

fn write_iter<T: Display>(
    f: &mut std::fmt::Formatter<'_>,
    list: &mut dyn Iterator<Item = T>,
    len: usize,
) -> std::fmt::Result {
    for (index, element) in list.enumerate() {
        write!(f, "{}", element)?;
        if index != len - 1 {
            write!(f, ", ")?;
        }
    }

    Ok(())
}
