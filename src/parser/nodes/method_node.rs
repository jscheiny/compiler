use crate::{
    checker::{EnumMethod, Scope, StructMember, StructMemberType, TypeResolver},
    parser::{FunctionNode, Identified, Node},
};

pub struct MethodNode {
    pub public: bool,
    pub function: Node<FunctionNode>,
}

impl MethodNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> Box<Scope> {
        self.function.check(types, scope)
    }

    pub fn resolve_struct_method(&self, types: &TypeResolver) -> StructMember {
        let function_type = self.function.get_type(types).clone();
        StructMember {
            public: self.public,
            member_type: StructMemberType::Method(function_type),
        }
    }

    pub fn resolve_enum_method(&self, types: &TypeResolver) -> EnumMethod {
        let function_type = self.function.get_type(types).clone();
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
