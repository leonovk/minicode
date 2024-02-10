use crate::opcode::OpCode;
use crate::opcode::OpCode::*;
use crate::opcode::ValueType::*;

// [] a
pub fn appropriation_array(data: Vec<&str>) -> OpCode {
    if data.len() < 2 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let value_name = data[1].to_string();

    Create(value_name, Arr(Vec::new()))
}

// []< a b
pub fn push_array(data: Vec<&str>) -> OpCode {
    if data.len() != 3 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let arr_name = data[1].to_string();
    let value_name = data[2].to_string();

    ArrayPush(arr_name, value_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_iappropriation_array_one() {
        let data = vec!["[]", "a"];
        let result = appropriation_array(data);

        assert_eq!(result, Create("a".to_string(), Arr(vec![])));
    }

    #[test]
    fn test_iappropriation_array_two() {
        let data = vec!["[]"];
        let result = appropriation_array(data);

        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }

    #[test]
    fn test_push_array_one() {
        let data = vec!["[]<", "a", "b"];
        let result = push_array(data);

        assert_eq!(result, ArrayPush("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_push_array_two() {
        let data = vec!["[]<", "a", "b", "dsa"];
        let result = push_array(data);

        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }
}
