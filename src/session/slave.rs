use std::sync::mpsc::{Receiver, channel};

use rdev::EventType;

use crate::session::backend::tcp::TcpSlave;

pub struct Slave {
    backend: TcpSlave,
    receiver: Receiver<EventType>, // <- Naive approach; TODO : synchronize time
}

impl Slave {
    pub async fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            backend: TcpSlave::new(sender).await,
            receiver,
        }
    }

    pub async fn run(self) {
        tokio::task::spawn(self.backend.run());
        for input in self.receiver.iter() {
            println!("{input:?}");
            #[cfg(not(feature = "without_simulation"))]
            rdev::simulate(&input).unwrap()
        }
    }
}
