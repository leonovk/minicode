#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum ValueType {
    Int(f64),
    Line(String),
}

#[derive(PartialEq, Debug)]
pub enum OperationType {
    Increment,
    Decrement,
    Multiplication,
    Division,
}

#[derive(PartialEq, Debug)]
pub enum ComparisonOperators {
    Equals,
    NotEquals,
    More,
    Less,
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Create(String, ValueType),
    Print(String),
    PrintFile(String, String),
    Operation(String, OperationType, ValueType),
    Condition(String, ValueType, ComparisonOperators, usize),
    ErrorCode(String),
    Execute(String, String, Vec<String>),
    Include(String),
    EmptyLine,
}
