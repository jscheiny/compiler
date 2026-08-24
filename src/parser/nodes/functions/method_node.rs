use crate::{
    checker::{Scope, ScopeType},
    parser::{FunctionNode, Node},
};

#[derive(Clone, Copy, PartialEq, Eq)]
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
        match self.instance_kind {
            MethodInstanceKind::Instance => self.function.check(scope),
            MethodInstanceKind::Static => {
                scope.nest(ScopeType::StaticMethod, |scope| self.function.check(scope))
            }
        }
    }

    pub fn name(&self) -> &String {
        self.function.name()
    }
}
