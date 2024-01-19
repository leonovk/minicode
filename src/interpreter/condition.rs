use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

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
