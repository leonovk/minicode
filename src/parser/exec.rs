pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;

pub fn exec(data: Vec<&str>) -> OpCode {
    if data.len() < 3 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let value_name = data[1].to_string();
    let command_name = data[2].to_string();
    let mut args: Vec<String> = Vec::new();

    for arg in data.into_iter().skip(3) {
        args.push(arg.to_string());
    }

    Execute(value_name, command_name, args)
}

#[cfg(test)]
mod tests {
    use super::exec;
    use crate::opcode::OpCode::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_exec_one() {
        let data = vec!["&", "a", "minicode", "--version"];
        let result = exec(data);
        assert_eq!(
            result,
            Execute(
                "a".to_string(),
                "minicode".to_string(),
                vec!["--version".to_string()]
            )
        );
    }

    #[test]
    fn test_exec_two() {
        let data = vec!["&", "a", "minicode", "-p", "file.txt"];
        let result = exec(data);
        assert_eq!(
            result,
            Execute(
                "a".to_string(),
                "minicode".to_string(),
                vec!["-p".to_string(), "file.txt".to_string()]
            )
        );
    }

    #[test]
    fn test_exec_three() {
        let data = vec!["&", "a", "minicode"];
        let result = exec(data);
        assert_eq!(
            result,
            Execute("a".to_string(), "minicode".to_string(), vec![])
        );
    }

    #[test]
    fn test_exec_four() {
        let data = vec!["&", "a"];
        let result = exec(data);
        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }
}
