use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType;
use std::collections::HashMap;
mod calculate;
mod condition;
mod create;
mod print_file;
mod print_value;
use calculate::calculate;
use condition::condition;
use create::create;
use print_file::print_file;
use print_value::print_value;

pub fn exegete(operations: Vec<OpCode>) {
    if operations.is_empty() {
        return;
    }

    let code_max_point = operations.len() - 1;
    let mut pointer: usize = 0;
    let mut addresses: HashMap<&String, ValueType> = HashMap::new();

    while pointer <= code_max_point {
        let operation = &operations[pointer];

        match operation {
            Create(k, v) => create(k, v, &mut addresses),
            Print(k) => print_value(k, &addresses),
            Operation(k, o, v) => calculate(k, o, v, &mut addresses),
            ErrorCode(e) => {
                println!("{} - line - {}", e, pointer + 1);
                break;
            }
            Condition(k, v, b, p) => {
                let result = condition(k, b, v, &addresses);
                if result {
                    new_pointer(&mut pointer, p);
                }
            }
            PrintFile(key, path) => print_file(key, path, &addresses),
            EmptyLine => {}
        }

        pointer += 1;
    }
}

fn new_pointer(pointer: &mut usize, new: &usize) {
    if *new > 2 {
        *pointer = *new - 2;
    } else {
        *pointer = 0;
    }
}

#[test]
fn test_new_pointer() {
    let mut pointer: usize = 1;
    new_pointer(&mut pointer, &1);
    assert_eq!(pointer, 0);

    let mut pointer: usize = 1;
    new_pointer(&mut pointer, &3);
    assert_eq!(pointer, 1);
}
