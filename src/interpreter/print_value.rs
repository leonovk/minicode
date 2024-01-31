use super::opcode_result_type::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn print_value(
    key: &String,
    storage: &HashMap<&String, ValueType>,
) -> Result<OpCodeResultType, String> {
    let value = storage.get(key);

    match value {
        Some(s) => match s {
            Int(i) => println!("{}", i),
            Line(s) => println!("{}", s),
            Arr(_a) => return Err("you can't print an array".to_string()),
        },
        None => println!("{}", key),
    };

    Ok(OpCodeResultType::Empty)
}
