use crate::{
    checker::Scope,
    parser::{FunctionNode, Node},
};

#[derive(Clone, Copy)]
pub enum MethodInstanceKind {
    Instance,
    Static,
}

pub struct MethodNode {
    pub public: bool,
    pub instance_kind: MethodInstanceKind,
    pub function: Node<FunctionNode>,
}

impl MethodNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        self.function.check(scope)
    }
}
