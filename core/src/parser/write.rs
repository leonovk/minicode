use crate::opcode::OpCode;
use crate::opcode::OpCode::*;

pub fn file_write(data: Vec<&str>) -> OpCode {
    if data.len() != 3 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    PrintFile(data[2].to_string(), data[1].to_string())
}

#[cfg(test)]
mod tests {
    use super::file_write;
    use crate::opcode::OpCode::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_file_write_one() {
        let data = vec![">>", "path", "value"];
        let result = file_write(data);
        assert_eq!(result, PrintFile("value".to_string(), "path".to_string()));
    }

    #[test]
    fn test_file_write_two() {
        let data = vec![">>", "path"];
        let result = file_write(data);
        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }
}
