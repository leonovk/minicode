use crate::opcode::OpCode;
use crate::opcode::OpCode::*;

pub fn print(data: Vec<&str>) -> OpCode {
    if data.len() < 2 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let value: String = data.into_iter().skip(1).collect::<Vec<&str>>().join(" ");

    Print(value)
}

#[cfg(test)]
mod tests {
    use super::print;
    use crate::opcode::OpCode::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_print_one() {
        let data = vec!["p", "a", "b", "c"];
        let result = print(data);
        assert_eq!(result, Print("a b c".to_string()));
    }

    #[test]
    fn test_print_two() {
        let data = vec!["p", "a"];
        let result = print(data);
        assert_eq!(result, Print("a".to_string()));
    }

    #[test]
    fn test_print_three() {
        let data = vec!["p"];
        let result = print(data);
        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }
}
