pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;

pub fn send_tcp(_data: Vec<&str>) -> OpCode {
    SendTcp("adr".to_string(), "mes".to_string())
}
