use crate::{
    checker::{EnumMethod, StructMember, StructMemberType, TypeResolver},
    parser::{FunctionParseNode, ParseNode, TokenSpan, Traverse},
};

pub struct MethodParseNode {
    pub public: bool,
    pub function: ParseNode<FunctionParseNode>,
}

impl Traverse for MethodParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.function.traverse("Method.function", visit);
    }
}

impl MethodParseNode {
    pub fn resolve_struct_method(&self, types: &TypeResolver) -> StructMember {
        let function_type = self.function.value.resolve_type(types);
        StructMember {
            public: self.public,
            member_type: StructMemberType::Method(function_type),
        }
    }

    pub fn resolve_enum_method(&self, types: &TypeResolver) -> EnumMethod {
        let function_type = self.function.value.resolve_type(types);
        EnumMethod {
            public: self.public,
            function_type,
        }
    }
}
