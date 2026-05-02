use crate::{checker::Type, parser::TokenSpan};

// TODO unify the API here and maybe a better name also rename file
pub trait Types {
    fn get_type_id(&self, name: &str) -> Option<usize>;
    fn get_type(&self, name: &str) -> Option<Type>;
    fn return_type(&self) -> Option<&Type>;
    fn get_self_type(&self) -> Option<Type>;
    fn print_error(&self, span: TokenSpan, message: &str, inline_message: &str);
}
