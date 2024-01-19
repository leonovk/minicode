use crate::opcode::OperationType;
use crate::opcode::OperationType::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

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
