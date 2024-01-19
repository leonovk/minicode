use crate::opcode::OpCode;
use crate::opcode::OpCode::*;

pub fn condition(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();
    let true_or_false = match data[2] {
        "=" => true,
        "!" => false,
        _ => return ErrorCode("wrong condition".to_string()),
    };

    let target_value = match data[3].parse::<i64>() {
        Ok(parsed) => parsed,
        Err(_) => return ErrorCode("wrong target value for operation".to_string()),
    };

    let target_pointer = match data[4].parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => return ErrorCode("wrong target pointer".to_string()),
    };

    Condition(value_name, target_value, true_or_false, target_pointer)
}
