use super::core::*;
use super::*;

use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;

/// Here the Block implementations and a factory to build the various blocks
/// starting from configuration

pub fn build_block(block_def: ModelDefBlock) -> Result<BoxedBlock, String> {
    debug!("{:?}", block_def);
    let id = block_def.id;
    match block_def.configuration {
        ModelDefBlockConfig::EventGenerator(config) => Ok(Box::new(
                EventGenerator::new(id, config)
        )),
        ModelDefBlockConfig::LoggingSink => Ok(Box::new(LoggingSink::new(id)))
    }
}

struct EventGenerator {
    id: String,
    system_port: InputPort,
    out_port: OutputPort
}

impl EventGenerator {
    pub fn new(id: String, config: ModelDefEventGeneratorConfiguration) -> Self {
        debug!("EventGenerator: {:?}", config);
        EventGenerator {
            id,
            system_port: InputPort::new(),
            out_port: OutputPort::new()
        }
    }
}

impl Block for EventGenerator {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn input_ports(&self) -> HashMap<String, &InputPort> {
        HashMap::new()
    }

    fn output_ports(&self) -> HashMap<String, &OutputPort> {
        let mut map = HashMap::new();
        map.insert("out".to_string(), &self.out_port);
        map
    }

    fn init(&self) {
    }

    fn shutdown(&self) {
    }

    fn system_port(&self) -> &InputPort {
        &self.system_port
    }

    fn thread_executor(&self) {
        loop {
            match self.system_port.receive() {
                Ok(system_event) => match system_event.event_type {
                    EventType::Start => debug!("EventGenerator {} Start Event", self.id()),
                    EventType::Stop => {
                            debug!("EventGenerator {} Stop Event", self.id());
                            return
                        },
                    _ => ()
                },
                Err(TryRecvError::Empty) => self.process(),
                Err(TryRecvError::Disconnected) => return
            }
            thread::yield_now();
        }
    }
}

impl EventGenerator {
    fn process(&self) {
    }
}

struct LoggingSink {
    id: String,
    system_port: InputPort,
    in_port: InputPort
}

impl LoggingSink {
    pub fn new(id: String) -> Self {
        debug!("LoggingSink");
        let mut in_ports = HashMap::new();
        in_ports.insert("in".to_string(), InputPort::new());
        LoggingSink {
            id,
            system_port: InputPort::new(),
            in_port: InputPort::new()
        }
    }
}

impl Block for LoggingSink {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn input_ports(&self) -> HashMap<String, &InputPort> {
        let mut map = HashMap::new();
        map.insert("in".to_string(), &self.in_port);
        map
    }

    fn output_ports(&self) -> HashMap<String, &OutputPort> {
        HashMap::new()
    }

    fn init(&self) {
    }

    fn shutdown(&self) {
    }

    fn system_port(&self) -> &InputPort {
        &self.system_port
    }

    fn thread_executor(&self) {
        loop {
            match self.system_port.receive() {
                Ok(system_event) => match system_event.event_type {
                    EventType::Start => debug!("LoggingSink {} Start Event", self.id()),
                    EventType::Stop => {
                            debug!("LoggingSink {} Stop Event", self.id());
                            return
                        },
                    _ => ()
                },
                Err(TryRecvError::Empty) => thread::yield_now(),
                Err(TryRecvError::Disconnected) => return
            }
        }
    }
}
