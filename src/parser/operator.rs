pub enum PrefixOperator {
    Negative, // not
    SelfRef,  // @
}

pub enum BinaryOperator {
    Add,                 // +
    AddAssign,           // +=
    Subtract,            // -
    SubtractAssign,      // -=
    Multiply,            // *
    MultiplyAssign,      // *=
    Divide,              // /
    DivideAssign,        // /=
    Mod,                 // %
    ModAssign,           // %=
    Assign,              // =
    Equal,               // ==
    LessThan,            // <
    LessThanOrEqual,     // <=
    GreaterThan,         // >
    GreaterThanOrEqual,  // >=
    Access,              // .
    FunctionApplication, // =>
    And,                 // and
    Or,                  // or
}

pub enum PostfixOperator {
    NullShortCircuit, // ?
}
