use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};


#[derive(Copy, Clone, Debug)]
pub enum EventType {
    Start,
    Stop,
    Trigger
}

#[derive(Copy, Clone, Debug)]
pub struct Event {
    pub timestamp: f64,
    pub corrected_timestamp: f64,
    pub event_type: EventType
    // TODO event metadata
    // TODO event data
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Event {
            timestamp: 0.0,
            corrected_timestamp: 0.0,
            event_type
        }
    }
}

pub struct BlockPorts {
    pub system_port: InputPort,
    pub input_ports: HashMap<String, InputPort>,
    pub output_ports: HashMap<String, OutputPort>
}

pub trait Block {
    fn id(&self) -> &str;
    fn thread_executor(&self, ports: BlockPorts);
    fn init(&self);
    fn shutdown(&self);
}

pub type BoxedBlock = Box<dyn Block + Send>;

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

    pub fn set_receiver(&mut self, receiver: Receiver<Event>) {
        self.receiver = Some(receiver);
    }

    pub fn receive(&self) -> Result<Event, TryRecvError> {
        match &self.receiver {
            Some(receiver) => receiver.try_recv(),
            None => Err(TryRecvError::Disconnected)
        }
    }
}
