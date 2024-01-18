pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;
pub use crate::opcode::ValueType::*;

pub fn get_opcode(line: &String) -> OpCode {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts[0] == ">" {
        let value_name = parts[1].to_string();
        let value: String = parts.into_iter().skip(2).collect::<Vec<&str>>().join(" ");
        match value.parse::<i64>() {
            Ok(parsed) => Create(value_name, Int(parsed)),
            Err(_e) => Create(value_name, Line(value)),
        }
    } else if parts[0] == "p" {
        let value_name = parts[1].to_string();
        Print(value_name)
    } else {
        ErrorCode("no command".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::opcode_parser::*;

    #[test]
    fn test_get_opcode() {
        let ex1 = "> a 32".to_string();

        assert_eq!(get_opcode(&ex1), Create("a".to_string(), Int(32)));
    }
}
