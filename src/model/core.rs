use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};


#[derive(Copy, Clone)]
pub enum EventType {
    Trigger
}

#[derive(Copy, Clone)]
pub struct Event {
    timestamp: f64,
    corrected_timestamp: f64,
    event_type: EventType
    // event metadata
    // event data
}

pub trait Block {
    fn id(&self) -> &str;
    fn input_ports(&self) -> &HashMap<String, InputPort>;
    fn output_ports(&self) -> &HashMap<String, OutputPort>;
    fn init(&self);
    fn shutdown(&self);
}

pub struct OutputPort {
    senders: Vec<Sender<Event>>
}

pub struct InputPort {
    receiver: Option<Receiver<Event>>
}

impl OutputPort {
    pub fn new() -> Self {
        OutputPort {
            senders: Vec::new()
        }
    }

    pub fn add_sender(&mut self, sender: Sender<Event>) {
        self.senders.push(sender)
    }

    pub fn clear(&mut self) {
        self.senders = Vec::new();
    }

    pub fn send(&self, event: Event) {
        for sender in &self.senders {
            sender.send(event);
        }
    }
}

impl InputPort {
    pub fn new() -> Self {
        InputPort {
            receiver: None
        }
    }

    pub fn set_receiver(&mut self, receiver: Receiver<Event>) -> Result<(), String> {
        if self.receiver.is_some() {
            Err("Receiver already set".to_string())
        } else {
            self.receiver = Some(receiver);
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.receiver = None;
    }


    pub fn receive(&self) -> Result<Event, TryRecvError> {
        match &self.receiver {
            Some(receiver) => receiver.try_recv(),
            None => Err(TryRecvError::Disconnected)
        }
    }
}
