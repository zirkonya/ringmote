use rdev::EventType;
use std::sync::mpsc::{Receiver, Sender};
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

    async fn read_message(&mut self, master: &mut TcpStream) -> tokio::io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        master.read_buf(&mut buffer).await?;
        Ok(buffer)
    }

    pub async fn run(mut self) {
        loop {
            if let Ok((mut master, _addr)) = self.listener.accept().await {
                while let Ok(message) = self.read_message(&mut master).await {
                    println!("{}", String::from_utf8_lossy(&message));
                    match serde_json::from_slice(&message) {
                        Ok(input) => {
                            if let Err(err) = self.sender.send(input) {
                                eprintln!("{err:?}");
                            }
                        }
                        Err(err) => eprintln!("{err:?}"),
                    }
                }
            }
        }
    }
}

pub struct TcpMaster {
    stream: TcpStream,
    receiver: Receiver<EventType>,
}

impl TcpMaster {
    pub async fn new(receiver: Receiver<EventType>) -> Self {
        Self {
            stream: TcpStream::connect(format!("localhost:{DEFAULT_TCP_PORT}"))
                .await
                .unwrap(),
            receiver,
        }
    }

    async fn send_message(&mut self, message: &[u8]) {
        self.stream.write_all(message).await.unwrap();
        self.stream.flush().await.unwrap();
    }

    fn serialize_input(input: &EventType) -> Vec<u8> {
        serde_json::to_vec(&input).unwrap()
    }

    pub async fn run(mut self) {
        loop {
            let input = self.receiver.recv().expect("ERROR");
            let message = Self::serialize_input(&input);
            self.send_message(&message).await
        }
    }
}
