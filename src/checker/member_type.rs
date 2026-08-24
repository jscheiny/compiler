use crate::{checker::Type, parser::MethodInstanceKind};

pub trait MemberType {
    fn is_public(&self) -> bool;
    fn instance_kind(&self) -> MethodInstanceKind;
    fn get_type(&self) -> Type;
}
