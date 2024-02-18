use crate::network::send_tcp_message;
use crate::opcode::OpCodeResultType;
use crate::opcode::OpCodeResultType::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn send_tcp(
    addr: &String,
    mes: &String,
    storage: &HashMap<&String, ValueType>,
) -> Result<OpCodeResultType, String> {
    let binding_addr = Line(addr.to_string());
    let binding_mes = Line(addr.to_string());
    let address = storage.get(addr).unwrap_or(&binding_addr);
    let message = storage.get(mes).unwrap_or(&binding_mes);

    if !all_lines(address, message) {
        return Err("Only strings can be either an address or a message".to_string());
    }

    let data = unpack_strings(address, message);
    let result = send_tcp_message(data.0, data.1);

    // +_+
    match result {
        Ok(_ok) => Ok(Empty),
        Err(er) => Err(er),
    }
}

fn all_lines(adr: &ValueType, mes: &ValueType) -> bool {
    match (adr, mes) {
        (ValueType::Line(_), ValueType::Line(_)) => true,
        _ => false,
    }
}

fn unpack_strings<'a>(adr: &'a ValueType, mes: &'a ValueType) -> (&'a String, &'a String) {
    let adr_string = match adr {
        Line(line) => line,
        _ => panic!("address parsing error"),
    };
    let mes_string = match mes {
        Line(line) => line,
        _ => panic!("message parsing error"),
    };

    (adr_string, mes_string)
}
