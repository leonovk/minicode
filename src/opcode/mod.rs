#[derive(PartialEq, Debug)]
pub enum ValueType {
    Int(i64),
    Line(String)
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Create(String, ValueType),
    Print(String),
    ErrorCode(String)
}
