use crate::{
    checker::{EnumMethod, StructMember, StructMemberType, TypeResolver},
    parser::{FunctionParseNode, Identified, ParseNode},
};

pub struct MethodParseNode {
    pub public: bool,
    pub function: ParseNode<FunctionParseNode>,
}

impl MethodParseNode {
    pub fn resolve_struct_method(&mut self, types: &TypeResolver) -> StructMember {
        let function_type = self.function.get_type(types).clone();
        StructMember {
            public: self.public,
            member_type: StructMemberType::Method(function_type),
        }
    }

    pub fn resolve_enum_method(&mut self, types: &TypeResolver) -> EnumMethod {
        let function_type = self.function.get_type(types).clone();
        EnumMethod {
            public: self.public,
            function_type,
        }
    }
}

impl Identified for MethodParseNode {
    fn id(&self) -> &String {
        self.function.id()
    }
}
