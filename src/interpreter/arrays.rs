use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn push<'a>(key: &'a String, value: &'a String, target: &mut HashMap<&'a String, ValueType>) {
    let second_value = match target.get(value) {
        Some(some) => some.clone(),
        None => match value.parse::<f64>() {
            Ok(i) => Int(i),
            Err(_) => Line(value.to_string()),
        },
    };

    let first_value: &mut Vec<ValueType> = match target.get_mut(key) {
        Some(some) => match some {
            Arr(a) => a,
            _ => panic!("not is array"),
        },
        None => panic!("not is array"),
    };

    first_value.push(second_value);
}
