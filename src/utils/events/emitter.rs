use std::sync::mpsc::Sender;
use crate::TxData;

pub struct Emitter {
    event_sender: Sender<TxData>,
}

impl Emitter {
    pub fn new(event_sender: Sender<TxData>) -> Self {
        Emitter { event_sender }
    }

    pub fn emit_event(&self, event: TxData) {
        self.event_sender.send(event).expect("Failed to send event");
    }
}