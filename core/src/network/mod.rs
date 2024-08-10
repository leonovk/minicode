use std::io::Write;
use std::net::TcpStream;

pub fn send_tcp_message(addr: &String, message: &String) -> std::io::Result<()> {
    // Подключаемся к серверу по указанному IP и порту
    let mut stream = TcpStream::connect(addr)?;

    // Отправляем сообщение
    stream.write_all(message.as_bytes())?;

    Ok(())
}
