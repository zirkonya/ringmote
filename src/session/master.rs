use std::sync::mpsc::{Sender, channel};

use rdev::{Event, EventType};

use crate::session::backend::tcp::TcpMaster;

pub struct Master {
    backend: TcpMaster,
    sender: Sender<EventType>,
}

impl Master {
    pub async fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            backend: TcpMaster::new(receiver).await,
            sender,
        }
    }

    pub async fn run(self) {
        tokio::task::spawn(self.backend.run());
        rdev::listen(move |Event { event_type, .. }| self.sender.send(event_type).expect("ERROR"))
            .unwrap()
    }
}
