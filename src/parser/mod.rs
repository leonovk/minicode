mod appropriation;
mod calculation;
mod condition;
mod file;
mod opcode_parser;
mod print;
mod user_var;
use crate::opcode::*;

pub fn parse(lines: &Vec<String>) -> Vec<OpCode> {
    let mut result: Vec<OpCode> = Vec::new();

    for line in lines {
        if line == "" {
            continue;
        }

        let opcode = opcode_parser::get_opcode(line);
        result.push(opcode);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::opcode::OpCode::*;
    use crate::opcode::OperationType::*;
    use crate::opcode::ValueType::*;
    use crate::parser::*;

    #[test]
    fn test_parse_hello_world() {
        let input = vec![
            "> a 231".to_string(),
            "> b 'hello world'".to_string(),
            "p a".to_string(),
            "p b".to_string(),
            "dsad".to_string(),
            "= a + 1".to_string(),
            "= a - 1".to_string(),
            "? a ! 5 3".to_string(),
            "? a = 5 3".to_string(),
        ];

        let result = parse(&input);

        let expect_result = vec![
            Create("a".to_string(), Int(231)),
            Create("b".to_string(), Line("'hello world'".to_string())),
            Print("a".to_string()),
            Print("b".to_string()),
            ErrorCode("Could not recognize the command".to_string()),
            Operation("a".to_string(), Increment, 1),
            Operation("a".to_string(), Decrement, 1),
            Condition("a".to_string(), Int(5), false, 3),
            Condition("a".to_string(), Int(5), true, 3),
        ];

        let mut i = 0;

        for opcode in result {
            assert_eq!(opcode, expect_result[i]);
            i += 1;
        }
    }
}
