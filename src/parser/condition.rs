use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType::*;

pub fn condition(data: Vec<&str>) -> OpCode {
    if data.len() != 5 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let value_name = data[1].to_string();

    let true_or_false = match data[2] {
        "=" => true,
        "!" => false,
        _ => return ErrorCode("wrong condition".to_string()),
    };

    let target_pointer = match data[4].parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => return ErrorCode("wrong target pointer".to_string()),
    };

    let target_value = match data[3].parse::<i64>() {
        Ok(parsed) => Int(parsed),
        Err(_) => Line(data[3].to_string()),
    };

    Condition(value_name, target_value, true_or_false, target_pointer)
}

#[cfg(test)]
mod tests {
    use super::condition;
    use crate::opcode::OpCode::*;
    use crate::opcode::ValueType::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_condition_success_one() {
        let input = vec!["?", "a", "!", "3", "5"];
        let result = condition(input);
        assert_eq!(result, Condition("a".to_string(), Int(3), false, 5));
    }

    #[test]
    fn test_condition_success_two() {
        let input = vec!["?", "a", "=", "3", "5"];
        let result = condition(input);
        assert_eq!(result, Condition("a".to_string(), Int(3), true, 5));
    }

    #[test]
    fn test_condition_success_three() {
        let input = vec!["?", "a", "=", "3d", "5"];
        let result = condition(input);
        assert_eq!(
            result,
            Condition("a".to_string(), Line("3d".to_string()), true, 5)
        );
    }

    #[test]
    fn test_condition_success_four() {
        let input = vec!["?", "a", "!", "3d", "5"];
        let result = condition(input);
        assert_eq!(
            result,
            Condition("a".to_string(), Line("3d".to_string()), false, 5)
        );
    }

    #[test]
    fn test_condition_fail_one() {
        let input = vec!["?", "a", "!", "3"];
        let result = condition(input);
        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }

    #[test]
    fn test_condition_fail_two() {
        let input = vec!["?", "a", "asd", "3", "5"];
        let result = condition(input);
        assert_eq!(result, ErrorCode("wrong condition".to_string()));
    }

    #[test]
    fn test_condition_fail_three() {
        let input = vec!["?", "a", "=", "3", "5sa"];
        let result = condition(input);
        assert_eq!(result, ErrorCode("wrong target pointer".to_string()));
    }
}
