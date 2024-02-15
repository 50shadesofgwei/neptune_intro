use std::sync::mpsc::Receiver;
use crate::TxData;

pub fn event_listener(event_receiver: Receiver<TxData>) {
    for event in event_receiver {
        match event {
            TxData::tx_to_execute(data) => {
                println!("Data Updated event received with data: {}", data);
            },
        }
    }
}