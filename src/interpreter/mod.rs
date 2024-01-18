use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType;
use std::collections::HashMap;
mod code_operations;

pub fn exegete(operations: Vec<OpCode>) {
    let code_max_point = operations.len() - 1;
    let mut pointer: usize = 0;
    let mut addresses: HashMap<&String, ValueType> = HashMap::new();

    while pointer <= code_max_point {
        let operation = &operations[pointer];

        match operation {
            Create(k, v) => code_operations::create(k, v, &mut addresses),
            Print(k) => code_operations::print_value(k, &addresses),
            ErrorCode(e) => panic!("{}", e),
        }

        pointer += 1;
    }
}
