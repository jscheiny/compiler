use crate::parser::{FunctionSignatureNode, MethodInstanceKind};

pub struct MethodSignatureNode {
    pub instance_kind: MethodInstanceKind,
    pub signature: FunctionSignatureNode,
}
