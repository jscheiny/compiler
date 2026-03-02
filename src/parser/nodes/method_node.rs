use crate::{
    checker::{EnumMethod, Scope, StructMember, StructMemberType},
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

    pub fn resolve_struct_method(&self, scope: &Scope) -> StructMember {
        let function_type = self.function.get_type(scope).clone();
        StructMember {
            public: self.public,
            member_type: StructMemberType::Method(function_type),
        }
    }

    pub fn resolve_enum_method(&self, scope: &Scope) -> EnumMethod {
        let function_type = self.function.get_type(scope).clone();
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
