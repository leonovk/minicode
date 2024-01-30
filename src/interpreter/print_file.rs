use crate::files::write_content_to_file;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn print_file(key: &String, path: &String, storage: &HashMap<&String, ValueType>) {
    let binding = Line(key.to_string());
    let value = storage.get(key).unwrap_or(&binding);

    let printed_content = match value {
        Line(line) => line.to_string().replace("\\n", "\n"),
        Int(int) => int.to_string(),
        Arr(_arr) => panic!("you can't print an array"),
    };

    match write_content_to_file(&printed_content, path) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write file"),
    };
}
