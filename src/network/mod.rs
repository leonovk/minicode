use std::io::{Read, Write};
use std::net::TcpStream;

#[warn(dead_code)]
fn send_tcp_message(message: &String, addr: &String) -> Result<String, String> {
    let mut stream = TcpStream::connect(addr).map_err(|e| format!("Failed to connect: {}", e))?;

    stream
        .write_all(message.as_bytes())
        .map_err(|e| format!("Failed to write to stream: {}", e))?;

    let mut response = String::new();

    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(response)
}
