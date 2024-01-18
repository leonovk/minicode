use crate::opcode::OpCode;
use crate::opcode::OpCode::*;

pub fn exegete(operations: Vec<OpCode>) {
    for operation in operations {
        eval(operation);
    }
}

pub fn eval(operation: OpCode) {
    match operation {
        Create(_k, _v) => todo!(),
        Print(_k) => todo!(),
        ErrorCode(e) => panic!("{}", e)
    }
}
