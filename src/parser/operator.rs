#[derive(Debug)]
pub enum PrefixOperator {
    Negative,
    SelfRef,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    AddAssign,
    Subtract,
    SubtractAssign,
    Multiply,
    MultiplyAssign,
    Divide,
    DivideAssign,
    Mod,
    ModAssign,
    Assign,
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Access,
    NullShortCircuit,
    FunctionApplication,
}
