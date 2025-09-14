use std::sync::mpsc::Sender;

use rdev::EventType;
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub const DEFAULT_TCP_PORT: u16 = 3671;

pub struct TcpSlave {
    listener: TcpListener,
    sender: Sender<EventType>, // Naive approach; TODO : synchronize time
}

impl TcpSlave {
    pub async fn new(sender: Sender<EventType>) -> Self {
        Self {
            listener: TcpListener::bind(format!("localhost:{DEFAULT_TCP_PORT}"))
                .await
                .unwrap(),
            sender,
        }
    }

    async fn read_message(&mut self, master: &mut TcpStream) -> Vec<u8> {
        let mut buffer = Vec::new();
        master.read_buf(&mut buffer).await.unwrap();
        buffer
    }

    pub async fn run(mut self) {
        loop {
            if let Ok((mut master, _addr)) = self.listener.accept().await {
                let message = self.read_message(&mut master).await;
                println!(
                    "{}",
                    message
                        .iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                // DESERIALIZE
                self.sender
                    .send(EventType::KeyPress(rdev::Key::KeyA))
                    .unwrap();
                self.sender
                    .send(EventType::KeyRelease(rdev::Key::KeyA))
                    .unwrap();
            }
        }
    }
}

pub struct TcpMaster {
    stream: TcpStream,
}

impl TcpMaster {
    pub async fn new() -> Self {
        Self {
            stream: TcpStream::connect(format!("localhost:{DEFAULT_TCP_PORT}"))
                .await
                .unwrap(),
        }
    }

    async fn send_message(&mut self, message: &[u8]) {
        self.stream.write(message).await.unwrap();
        self.stream.flush().await.unwrap();
    }

    pub async fn run(mut self) {
        self.send_message(b"Hello World").await;
    }
}
