use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType::*;
use std::io;

pub fn user_var(data: Vec<&str>) -> OpCode {
    let mut input = String::new();
    let value_name = data[1].to_string();
    let output = data.into_iter().skip(2).collect::<Vec<&str>>().join(" ");
    println!("{}", output);
    io::stdin().read_line(&mut input).expect("cant read");
    match input.trim().parse::<f64>() {
        Ok(parsed) => Create(value_name, Int(parsed)),
        Err(_e) => Create(value_name, Line(input)),
    }
}
