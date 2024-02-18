pub use crate::opcode::OpCode;
pub use crate::opcode::OpCode::*;

pub fn send_tcp(data: Vec<&str>) -> OpCode {
    if data.len() < 3 {
        return ErrorCode("the operation is not specified correctly".to_string());
    }

    let addr = data[1].to_string();
    let message = data[2].to_string();

    SendTcp(addr, message)
}

#[cfg(test)]
mod tests {
    use super::send_tcp;
    use crate::opcode::OpCode::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_send_tcp_one() {
        let data = vec!["@", "a", "m"];
        let result = send_tcp(data);
        assert_eq!(result, SendTcp("a".to_string(), "m".to_string()));
    }

    #[test]
    fn test_send_tcp_two() {
        let data = vec!["@", "a", "m", "asdas", "asdsad"];
        let result = send_tcp(data);
        assert_eq!(result, SendTcp("a".to_string(), "m".to_string()));
    }

    #[test]
    fn test_send_tcp_three() {
        let data = vec!["@", "a"];
        let result = send_tcp(data);
        assert_eq!(
            result,
            ErrorCode("the operation is not specified correctly".to_string())
        );
    }
}
