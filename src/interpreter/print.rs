use crate::files::write_content_to_file;
use crate::opcode::OpCodeResultType;
use crate::opcode::OpCodeResultType::*;
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

    Ok(Empty)
}

pub fn print_file(
    key: &String,
    path: &String,
    storage: &HashMap<&String, ValueType>,
) -> Result<OpCodeResultType, String> {
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

pub fn print_error(file: &String, line: usize, error: &String) {
    println!("File -> {}", file);
    println!("Line # {}", line);
    println!("Error: {}", error);
}
