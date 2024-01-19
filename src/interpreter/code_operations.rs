use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use crate::opcode::OperationType::*;
use crate::opcode::OperationType;

use std::collections::HashMap;

pub fn create<'a>(key: &'a String, value: &ValueType, target: &mut HashMap<&'a String, ValueType>) {
    match value {
        Int(v) => target.insert(key, Int(*v)),
        Line(s) => target.insert(key, Line(s.to_string())),
    };
}

pub fn calculate<'a>(key: &'a String, o_type: &OperationType, value: &i64, target: &mut HashMap<&'a String, ValueType>) {
    let old_value = match target.get(key) {
        Some(s) => match s {
            Int(i) => i,
            Line(_s) => &0
        },
        None => &0
    };

    let new_value = match o_type {
        Increment => old_value + value,
        Decrement => old_value - value
    };

    target.insert(key, Int(new_value));
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
