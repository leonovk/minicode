#[derive(PartialEq, Debug)]
pub enum ValueType {
    Int(i64),
    Line(String),
}

#[derive(PartialEq, Debug)]
pub enum OperationType {
    Increment,
    Decrement,
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Create(String, ValueType),
    Print(String),
    Operation(String, OperationType, i64),
    Condition(String, ValueType, bool, usize),
    ErrorCode(String),
}
