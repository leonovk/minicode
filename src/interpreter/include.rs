use crate::code_runner::run;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;
use std::thread;

pub fn include(
    file: &String,
    args: &Vec<String>,
    storage: &HashMap<&String, ValueType>,
    stream: &bool,
) {
    let mut result_args_value = Vec::new();

    for arg in args {
        match storage.get(arg) {
            None => result_args_value.push(arg.to_string()),
            Some(s) => match s {
                Line(l) => result_args_value.push(l.to_string()),
                Int(i) => result_args_value.push(i.to_string()),
            },
        };
    }

    if *stream {
        let file_clone = file.clone();
        thread::spawn(move || {
            run(file_clone, result_args_value);
        });
    } else {
        run(file.to_string(), result_args_value);
    }
}
