use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;
use std::process::Command;

use super::OpCodeResultType;

pub fn execute<'a>(
    key: &'a String,
    command: &String,
    args: &Vec<String>,
    target: &mut HashMap<&'a String, ValueType>,
) -> Result<OpCodeResultType, String> {
    let mut command = Command::new(command);

    for arg in args {
        command.arg(arg);
    }

    let output = command.output().expect("The command failed");
    let result = String::from_utf8(output.stdout).expect("Error reading output");

    target.insert(key, Line(result));

    Ok(OpCodeResultType::Empty)
}
