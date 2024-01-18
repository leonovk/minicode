pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;
pub use crate::opcode::ValueType::*;
use crate::files::get_content;

pub fn get_opcode(line: &String) -> OpCode {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts[0] == ">" {
        appropriation(parts)
    } else if parts[0] == "p" {
        Print(parts[1].to_string())
    } else if parts[0] == "f" {
        file(parts)
    } else {
        ErrorCode("no command".to_string())
    }
}

fn file(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();
    let content = get_content(&data[2].to_string());
    Create(value_name, Line(content))
}

fn appropriation(data: Vec<&str>) -> OpCode {
    let value_name = data[1].to_string();
    let value: String = data.into_iter().skip(2).collect::<Vec<&str>>().join(" ");
    match value.parse::<i64>() {
        Ok(parsed) => Create(value_name, Int(parsed)),
        Err(_e) => Create(value_name, Line(value)),
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
