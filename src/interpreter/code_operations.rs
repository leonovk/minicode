use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn create<'a>(key: &'a String, value: &ValueType, target: &mut HashMap<&'a String, ValueType>) {
    match value {
        Int(v) => target.insert(key, Int(*v)),
        Line(s) => target.insert(key, Line(s.to_string())),
    };
}

pub fn print_value(key: &String, storage: &HashMap<&String, ValueType>) {
    let value = storage.get(key);

    match value {
        Some(s) => match s {
            Int(i) => println!("{}", i),
            Line(s) => println!("{}", s),
        },
        None => panic!("not value"),
    };
}
