use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType::*;

pub fn appropriation(data: Vec<&str>) -> OpCode {
    if data.len() < 3 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let value_name = data[1].to_string();
    let value: String = data.into_iter().skip(2).collect::<Vec<&str>>().join(" ");
    match value.parse::<i64>() {
        Ok(parsed) => Create(value_name, Int(parsed)),
        Err(_e) => Create(value_name, Line(value)),
    }
}

#[cfg(test)]
mod tests {
    use super::appropriation;
    use crate::opcode::OpCode::*;
    use crate::opcode::ValueType::*;

    #[test]
    fn test_appropriation_int() {
        let data = vec![">", "a", "1"];
        let result = appropriation(data);
        assert_eq!(result, Create("a".to_string(), Int(1)));
    }

    #[test]
    fn test_appropriation_line() {
        let data = vec![">", "a", "1d"];
        let result = appropriation(data);
        assert_eq!(result, Create("a".to_string(), Line("1d".to_string())));
    }

    #[test]
    fn test_appropriation_long_line() {
        let data = vec![">", "a", "1d", "dsad", "123"];
        let result = appropriation(data);
        assert_eq!(result, Create("a".to_string(), Line("1d dsad 123".to_string())));
    }

    #[test]
    fn test_appropriation_edge_one() {
        let data = vec![">", "a"];
        let result = appropriation(data);
        assert_eq!(result, ErrorCode("the operation is not specified correctly".to_string()));
    }

    #[test]
    fn test_appropriation_edge_two() {
        let data = vec![">"];
        let result = appropriation(data);
        assert_eq!(result, ErrorCode("the operation is not specified correctly".to_string()));
    }
}
