pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;

pub fn include(data: Vec<&str>) -> OpCode {
    if data.len() != 2 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    Include(data[1].to_string())
}
