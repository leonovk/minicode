mod appropriation;
mod arrays;
mod calculation;
mod condition;
mod exec;
mod file;
mod include;
mod opcode_parser;
mod print;
mod sleep;
mod user_var;
mod write;
use crate::opcode::*;

pub fn parse(lines: &Vec<String>) -> Vec<OpCode> {
    let mut result: Vec<OpCode> = Vec::new();

    for line in lines {
        if line == "" {
            result.push(OpCode::EmptyLine);
            continue;
        }

        let opcode = opcode_parser::get_opcode(line);
        result.push(opcode);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::opcode::ComparisonOperators::*;
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
            Create("a".to_string(), Int(231.0)),
            Create("b".to_string(), Line("'hello world'".to_string())),
            Print("a".to_string()),
            Print("b".to_string()),
            ErrorCode("Could not recognize the command".to_string()),
            Operation("a".to_string(), Increment, Int(1.0)),
            Operation("a".to_string(), Decrement, Int(1.0)),
            Condition("a".to_string(), Int(5.0), NotEquals, 3),
            Condition("a".to_string(), Int(5.0), Equals, 3),
        ];

        let mut i = 0;

        for opcode in result {
            assert_eq!(opcode, expect_result[i]);
            i += 1;
        }
    }
}
