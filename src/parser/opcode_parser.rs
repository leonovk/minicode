pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;

use super::opcode_operations;

pub fn get_opcode(line: &String) -> OpCode {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts[0] == ">" {
        opcode_operations::appropriation(parts)
    } else if parts[0] == "p" {
        Print(parts[1].to_string())
    } else if parts[0] == "f" {
        opcode_operations::file(parts)
    } else if parts[0] == "$>" {
        opcode_operations::user_var(parts)
    } else if parts[0] == "=" {
        opcode_operations::calculation(parts)
    } else if parts[0] == "?" {
        opcode_operations::condition(parts)
    } else {
        ErrorCode("no command".to_string())
    }
}

#[cfg(test)]
mod tests {
    pub use crate::opcode::ValueType::*;
    use crate::parser::opcode_parser::*;

    #[test]
    fn test_get_opcode() {
        let ex1 = "> a 32".to_string();

        assert_eq!(get_opcode(&ex1), Create("a".to_string(), Int(32)));
    }
}
