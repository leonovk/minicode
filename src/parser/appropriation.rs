use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType::*;

pub fn appropriation(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();
    let value: String = data.into_iter().skip(2).collect::<Vec<&str>>().join(" ");
    match value.parse::<i64>() {
        Ok(parsed) => Create(value_name, Int(parsed)),
        Err(_e) => Create(value_name, Line(value)),
    }
}
