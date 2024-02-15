use std::sync::mpsc::{Sender, self};
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

pub fn get_emitter() -> Sender<TxData> {
    let (sender, _receiver) = mpsc::channel::<TxData>();
    sender
}