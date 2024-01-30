use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
mod arrays;
mod calculate;
mod condition;
mod create;
mod execute;
mod include;
mod print_file;
mod print_value;
use arrays::push;
use calculate::calculate;
use condition::condition;
use create::create;
use execute::execute;
use include::include;
use print_file::print_file;
use print_value::print_value;

pub fn exegete(operations: Vec<OpCode>, args: Vec<String>) {
    if operations.is_empty() {
        return;
    }

    let code_max_point = operations.len() - 1;
    let mut pointer: usize = 0;
    let mut addresses: HashMap<&String, ValueType> = HashMap::new();
    let mut parallel_computing = Vec::new();

    let c_args = parse_command_line_arguments(args);

    for (key, value) in &c_args {
        addresses.insert(key, value.clone());
    }

    while pointer <= code_max_point {
        let operation = &operations[pointer];

        match operation {
            Create(k, v) => create(k, v, &mut addresses),
            ArrayPush(k, v) => push(k, v, &mut addresses),
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
            Execute(k, c, arg) => execute(k, c, arg, &mut addresses),
            Include(p, a, s) => {
                let result = include(p, a, &addresses, s);

                match result {
                    Some(h) => parallel_computing.push(h),
                    None => {}
                }
            }
            Sleep(i) => thread::sleep(Duration::from_secs(*i)),
            EmptyLine => {}
        }

        pointer += 1;
    }

    for compute in parallel_computing {
        compute.join().expect("error");
    }
}

fn parse_command_line_arguments(args: Vec<String>) -> HashMap<String, ValueType> {
    let mut addresses: HashMap<String, ValueType> = HashMap::new();
    let mut i = 1;

    for arg in args {
        let arg_name = "ARG_".to_string() + &i.to_string();
        let arg_value = match arg.parse::<f64>() {
            Ok(f) => Int(f),
            Err(_) => Line(arg),
        };
        addresses.insert(arg_name, arg_value);
        i += 1;
    }

    addresses
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
