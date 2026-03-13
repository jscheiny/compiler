use crate::{
    checker::Scope,
    parser::{FunctionNode, Named, Node},
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

impl Named for MethodNode {
    fn id(&self) -> &String {
        self.function.id()
    }
}
