use crate::opcode::OpCode;
use crate::opcode::OpCode::*;

pub fn sleep(data: Vec<&str>) -> OpCode {
    if data.len() != 2 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    

    match data[1].to_string().parse::<u64>() {
        Ok(parsed) => Sleep(parsed),
        Err(_e) => ErrorCode("the operation is not specified correctly".to_string()),
    }
}
