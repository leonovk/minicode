use crate::files::get_content;
pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;
pub use crate::opcode::OperationType::*;
pub use crate::opcode::ValueType::*;
use std::io;

pub fn calculation(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();

    let op = match data[2] {
        "+" => Increment,
        "-" => Decrement,
        _ => return ErrorCode("wrong operation".to_string()),
    };

    match data[3].to_string().parse::<i64>() {
        Ok(parsed) => Operation(value_name, op, parsed),
        Err(_e) => ErrorCode("wrong type for operation".to_string()),
    }
}

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

pub fn user_var(data: Vec<&str>) -> OpCode {
    let mut input = String::new();
    let value_name = data[1].to_string();
    io::stdin().read_line(&mut input).expect("dont read");
    match input.trim().parse::<i64>() {
        Ok(parsed) => Create(value_name, Int(parsed)),
        Err(_e) => Create(value_name, Line(input)),
    }
}

pub fn file(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();
    let content = get_content(&data[2].to_string());
    Create(value_name, Line(content))
}

pub fn appropriation(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();
    let value: String = data.into_iter().skip(2).collect::<Vec<&str>>().join(" ");
    match value.parse::<i64>() {
        Ok(parsed) => Create(value_name, Int(parsed)),
        Err(_e) => Create(value_name, Line(value)),
    }
}
