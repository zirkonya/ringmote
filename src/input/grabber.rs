use rdev::Event;
use std::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub struct InputGrabber {
    sender: Sender<Event>,
}

impl InputGrabber {
    pub fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }

    pub fn grab_input(&self) -> JoinHandle<()> {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            rdev::listen(move |event| sender.send(event).expect("failed to send event"))
                .expect("failed to listen input");
        })
    }
}
