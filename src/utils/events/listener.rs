use std::{thread, sync::mpsc::Receiver};
use crate::TxData;

pub struct Listener {}

impl Listener {
    pub fn new(event_receiver: Receiver<TxData>) -> Self {
        std::thread::spawn(move || {
            for tx_data in event_receiver {
                println!("Received TxData: {:?}", tx_data);
            }
        });

        Self {}
    }
}