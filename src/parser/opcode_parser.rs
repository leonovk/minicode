pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;

use super::appropriation::appropriation;
use super::calculation::calculation;
use super::condition::condition;
use super::file::file;
use super::print::print;
use super::user_var::user_var;
use super::write::file_write;

pub fn get_opcode(line: &String) -> OpCode {
    let data: Vec<&str> = line.split_whitespace().collect();
    let command = data[0];

    match command {
        ">" => appropriation(data),
        "p" => print(data),
        "f" => file(data),
        "$>" => user_var(data),
        "=" => calculation(data),
        "?" => condition(data),
        ">>" => file_write(data),
        _ => ErrorCode("Could not recognize the command".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::opcode::ComparisonOperators::*;
    use crate::opcode::OperationType::*;
    pub use crate::opcode::ValueType::*;
    use crate::parser::opcode_parser::*;

    #[test]
    fn test_get_opcode_create() {
        let ex1 = "> a 32".to_string();

        assert_eq!(get_opcode(&ex1), Create("a".to_string(), Int(32.0)));
    }

    #[test]
    fn test_get_opcode_print() {
        let ex1 = "p a".to_string();

        assert_eq!(get_opcode(&ex1), Print("a".to_string()));
    }

    #[test]
    fn test_get_opcode_file() {
        let ex1 = "f a test/test_file.txt".to_string();

        assert_eq!(
            get_opcode(&ex1),
            Create("a".to_string(), Line("first\nsecond".to_string()))
        );
    }

    #[test]
    fn test_get_opcode_calc() {
        let ex1 = "= a + 1".to_string();

        assert_eq!(
            get_opcode(&ex1),
            Operation("a".to_string(), Increment, Int(1.0))
        );
    }

    #[test]
    fn test_get_opcode_condition() {
        let ex1 = "? a = 1 6".to_string();

        assert_eq!(
            get_opcode(&ex1),
            Condition("a".to_string(), Int(1.0), Equals, 6)
        );
    }
}
