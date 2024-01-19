use crate::opcode::OpCode;
use crate::opcode::OpCode::*;

pub fn print(data: Vec<&str>) -> OpCode {
    if data.len() != 2 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    Print(data[1].to_string())
}
