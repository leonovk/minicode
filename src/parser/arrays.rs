use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType::*;

// [] a
pub fn appropriation_array(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();

    Create(value_name, Arr(Vec::new()))
}

// []< a b
pub fn push_array(data: Vec<&str>) -> OpCode {
    if data.len() != 3 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let arr_name = data[1].to_string();
    let value_name = data[2].to_string();

    ArrayPush(arr_name, value_name)
}
