use crate::opcode::OpCode::*;
use crate::opcode::OpCode;

pub fn parse(_lines: &Vec<String>) -> Vec<OpCode> {
    vec![Create("dsda".to_string()), Mov(23)]
}
