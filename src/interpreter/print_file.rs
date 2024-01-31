use crate::files::write_content_to_file;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;
use super::opcode_result_type::*;

pub fn print_file(key: &String, path: &String, storage: &HashMap<&String, ValueType>) -> Result<OpCodeResultType, String> {
    let binding = Line(key.to_string());
    let value = storage.get(key).unwrap_or(&binding);

    let printed_content = match value {
        Line(line) => line.to_string().replace("\\n", "\n"),
        Int(int) => int.to_string(),
        Arr(_arr) => return Err("you can't print an array".to_string()),
    };

    match write_content_to_file(&printed_content, path) {
        Ok(_) => {}
        Err(_) => return Err("Failed to write file".to_string()),
    };

    Ok(OpCodeResultType::Empty)
}
