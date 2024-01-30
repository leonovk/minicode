pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;

pub fn include(data: Vec<&str>, stream: bool) -> OpCode {
    if data.len() < 2 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let value_name = data[1].to_string();

    let mut args: Vec<String> = Vec::new();

    for arg in data.into_iter().skip(2) {
        args.push(arg.to_string());
    }

    Include(value_name, args, stream)
}

#[cfg(test)]
mod tests {
    use super::include;
    use crate::opcode::OpCode::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_include_one() {
        let data = vec!["->", "a", "b", "c"];
        let result = include(data, false);
        assert_eq!(
            result,
            Include(
                "a".to_string(),
                vec!["b".to_string(), "c".to_string()],
                false
            )
        );
    }

    #[test]
    fn test_include_two() {
        let data = vec!["->", "a"];
        let result = include(data, true);
        assert_eq!(result, Include("a".to_string(), vec![], true));
    }

    #[test]
    fn test_include_three() {
        let data = vec!["p"];
        let result = include(data, false);
        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }
}
