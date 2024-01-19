use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn create<'a>(key: &'a String, value: &ValueType, target: &mut HashMap<&'a String, ValueType>) {
    match value {
        Int(v) => target.insert(key, Int(*v)),
        Line(s) => target.insert(key, Line(s.to_string())),
    };
}
