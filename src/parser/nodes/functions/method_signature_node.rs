use crate::parser::{FunctionSignatureNode, MethodInstanceType};

pub struct MethodSignatureNode {
    pub instance_type: MethodInstanceType,
    pub signature: FunctionSignatureNode,
}
