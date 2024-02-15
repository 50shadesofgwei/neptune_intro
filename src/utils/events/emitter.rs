use std::{sync::mpsc::{Sender, self}, error::Error};
use crate::TxData;

pub struct Emitter {
    event_sender: Sender<TxData>,
}

impl Emitter {
    pub fn new(event_sender: Sender<TxData>) -> Self {
        Emitter { event_sender }
    }
}

pub fn get_emitter() -> Sender<TxData> {
    let (sender, _receiver) = mpsc::channel::<TxData>();
    sender
}

pub fn emit_tx_data(emitter: Sender<TxData>, data_to_emit: TxData) -> Result<(), Box<dyn Error>> {
    emitter.send(data_to_emit).map_err(|e| e.into())
}