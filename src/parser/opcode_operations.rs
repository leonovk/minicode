use crate::files::get_content;
pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;
pub use crate::opcode::ValueType::*;
pub use crate::opcode::OperationType::*;
use std::io;

pub fn calculation(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();

    let op = match data[2] {
        "+" => Increment,
        "-" => Decrement,
        _ => return ErrorCode("dont Operation".to_string())
    };

    match data[3].to_string().parse::<i64>() {
        Ok(parsed) => Operation(value_name, op, parsed),
        Err(_e) => ErrorCode("dont Operation".to_string())
    }
}

pub fn user_var(data: Vec<&str>) -> OpCode {
    let mut input = String::new();
    let value_name = data[1].to_string();
    io::stdin().read_line(&mut input).expect("dont read");
    match input.parse::<i64>() {
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
