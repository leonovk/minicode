use crate::files::get_content;
pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;
pub use crate::opcode::ValueType::*;

pub fn file(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();
    let content = get_content(&data[2].to_string());
    Create(value_name, Line(content))
}
