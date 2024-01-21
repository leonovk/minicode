use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::OperationType::*;
use crate::opcode::ValueType::*;

pub fn calculation(data: Vec<&str>) -> OpCode {
    if data.len() != 4 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let value_name = data[1].to_string();

    let op = match data[2] {
        "+" => Increment,
        "-" => Decrement,
        "*" => Multiplication,
        "/" => Division,
        _ => return ErrorCode("wrong operation".to_string()),
    };

    match data[3].to_string().parse::<i64>() {
        Ok(parsed) => Operation(value_name, op, Int(parsed)),
        Err(_e) => Operation(value_name, op, Line(data[3].to_string())),
    }
}
