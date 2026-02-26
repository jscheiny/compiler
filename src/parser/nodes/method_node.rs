use crate::{
    checker::{EnumMethod, Scope, StructMember, StructMemberType, TypeResolver},
    lexer::SourceCode,
    parser::{FunctionNode, Identified, Node},
};

pub struct MethodNode {
    pub public: bool,
    pub function: Node<FunctionNode>,
}

impl MethodNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        self.function.check(scope)
    }

    pub fn resolve_struct_method(&self, types: &TypeResolver, source: &SourceCode) -> StructMember {
        let function_type = self.function.get_type(types, source).clone();
        StructMember {
            public: self.public,
            member_type: StructMemberType::Method(function_type),
        }
    }

    pub fn resolve_enum_method(&self, types: &TypeResolver, source: &SourceCode) -> EnumMethod {
        let function_type = self.function.get_type(types, source).clone();
        EnumMethod {
            public: self.public,
            function_type,
        }
    }
}

impl Identified for MethodNode {
    fn id(&self) -> &String {
        self.function.id()
    }
}
