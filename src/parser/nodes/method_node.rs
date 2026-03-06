use crate::{
    checker::Scope,
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
}

impl Identified for MethodNode {
    fn id(&self) -> &String {
        self.function.id()
    }
}
