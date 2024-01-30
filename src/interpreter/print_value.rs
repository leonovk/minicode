use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn print_value(key: &String, storage: &HashMap<&String, ValueType>) {
    let value = storage.get(key);

    match value {
        Some(s) => match s {
            Int(i) => println!("{}", i),
            Line(s) => println!("{}", s),
            Arr(_a) => panic!("you can't print an array"),
        },
        None => println!("{}", key),
    };
}
