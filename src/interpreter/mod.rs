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
mod opcode_result_type;
mod print;
use arrays::push;
use calculate::calculate;
use condition::condition;
use create::create;
use execute::execute;
use include::include;
use opcode_result_type::*;
use print::*;

pub fn exegete(operations: Vec<OpCode>, args: Vec<String>, file: &String) {
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

        let result = match operation {
            Create(k, v) => create(k, v, &mut addresses),
            ArrayPush(k, v) => push(k, v, &mut addresses),
            Print(k) => print_value(k, &addresses),
            Operation(k, o, v) => calculate(k, o, v, &mut addresses),
            ErrorCode(_e) => Err("error code 5".to_string()),
            Condition(k, v, b, p) => {
                pointer_updater(&mut pointer, p, condition(k, b, v, &addresses))
            }
            PrintFile(key, path) => print_file(key, path, &addresses),
            Execute(k, c, arg) => execute(k, c, arg, &mut addresses),
            Include(p, a, s) => {
                push_new_thread(&mut parallel_computing, include(p, a, &addresses, s))
            }
            SendTcp(_adr, _mes) => Ok(OpCodeResultType::Empty),
            Sleep(i) => go_sleep(i),
            EmptyLine => Ok(OpCodeResultType::Empty),
        };

        match result {
            Ok(_) => {}
            Err(e) => {
                print_error(file, pointer + 1, &e);
                return;
            }
        }

        pointer += 1;
    }

    for compute in parallel_computing {
        compute.join().expect("error 3");
    }
}

fn push_new_thread(
    threads: &mut Vec<thread::JoinHandle<()>>,
    result: Result<OpCodeResultType, String>,
) -> Result<OpCodeResultType, String> {
    match result {
        Ok(o) => match o {
            OpCodeResultType::Thread(th) => match th {
                Some(s) => {
                    threads.push(s);
                    Ok(OpCodeResultType::Empty)
                }
                None => Ok(OpCodeResultType::Empty),
            },
            _ => Err("error 4".to_string()),
        },
        Err(e) => Err(e),
    }
}

fn pointer_updater(
    pointer: &mut usize,
    new: &usize,
    result: Result<OpCodeResultType, String>,
) -> Result<OpCodeResultType, String> {
    match result {
        Ok(o) => match o {
            OpCodeResultType::Bool(b) => {
                if b {
                    new_pointer(pointer, new);
                    Ok(OpCodeResultType::Empty)
                } else {
                    Ok(OpCodeResultType::Empty)
                }
            }
            _ => Err("error 1".to_string()),
        },
        Err(e) => Err(e),
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

fn go_sleep(i: &u64) -> Result<OpCodeResultType, String> {
    thread::sleep(Duration::from_secs(*i));

    Ok(OpCodeResultType::Empty)
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
