use crate::opcode::OperationType;
use crate::opcode::OperationType::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;

use std::collections::HashMap;

pub fn create<'a>(key: &'a String, value: &ValueType, target: &mut HashMap<&'a String, ValueType>) {
    match value {
        Int(v) => target.insert(key, Int(*v)),
        Line(s) => target.insert(key, Line(s.to_string())),
    };
}

pub fn calculate<'a>(
    key: &'a String,
    o_type: &OperationType,
    value: &i64,
    target: &mut HashMap<&'a String, ValueType>,
) {
    let old_value = match target.get(key) {
        Some(s) => match s {
            Int(i) => i,
            Line(_s) => panic!("wrong type for calculate"),
        },
        None => panic!("not value for calculate"),
    };

    let new_value = match o_type {
        Increment => old_value + value,
        Decrement => old_value - value,
    };

    target.insert(key, Int(new_value));
}

pub fn condition(
    key: &String,
    true_or_false: &bool,
    target_value: &i64,
    storage: &HashMap<&String, ValueType>,
) -> bool {
    let value = storage.get(key);

    match value {
        Some(s) => match s {
            Int(i) => {
                if *true_or_false {
                    return i == target_value;
                } else {
                    return i != target_value;
                }
            }
            Line(_s) => panic!("condition - wrong type value"),
        },
        None => panic!("condition - not value"),
    };
}

pub fn print_value(key: &String, storage: &HashMap<&String, ValueType>) {
    let value = storage.get(key);

    match value {
        Some(s) => match s {
            Int(i) => println!("{}", i),
            Line(s) => println!("{}", s),
        },
        None => panic!("not value for print"),
    };
}
