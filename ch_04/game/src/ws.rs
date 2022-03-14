use std::net::TcpStream;
use tungstenite::{client::connect, stream::MaybeTlsStream, Message, WebSocket};

pub struct Connection {
    socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Connection {
    pub fn new() -> Self {
        Self { socket: None }
    }

    pub fn connect(&mut self, url: &str) {
        if let Ok((mut socket, _)) = connect(url) {
            if let MaybeTlsStream::Plain(s) = socket.get_mut() {
                s.set_nonblocking(true).unwrap();
            }

            self.socket = Some(socket);
        }
    }

    pub fn poll(&mut self) -> Option<Vec<u8>> {
        if let Some(socket) = &mut self.socket {
            if let Ok(msg) = socket.read_message() {
                if let Message::Binary(buf) = msg {
                    return Some(buf);
                }
            }
        }
        None
    }

    pub fn send(&mut self, msg: Vec<u8>) {
        if let Some(socket) = &mut self.socket {
            socket.write_message(Message::Binary(msg)).unwrap();
        }
    }
}
