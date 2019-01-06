use super::core::*;
use super::*;

use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;

/// Here the Block implementations and a factory to build the various blocks
/// starting from configuration

pub fn build_block(block_def: &ModelDefBlock) -> BoxedBlock {
    debug!("{:?}", block_def);
    let id = block_def.id.to_owned();
    match &block_def.configuration {
        ModelDefBlockConfig::EventGenerator(config) => 
            Box::new(EventGenerator::new(id, config.clone())),
        ModelDefBlockConfig::LoggingSink => Box::new(LoggingSink::new(id))
    }
}

struct EventGenerator {
    id: String,
    config: ModelDefEventGeneratorConfiguration
}

impl EventGenerator {
    pub fn new(id: String, config: ModelDefEventGeneratorConfiguration) -> Self {
        debug!("EventGenerator: {:?}", config);
        EventGenerator {
            id,
            config
        }
    }
}

impl Block for EventGenerator {
    fn id(&self) -> &str {
        &self.id
    }

    fn init(&self) {
    }

    fn shutdown(&self) {
    }

    fn thread_executor(&self, ports: BlockPorts) {
        let out_port = ports.output_ports.get("out").unwrap();
        loop {
            match ports.system_port.receive() {
                Ok(system_event) => match system_event.event_type {
                    EventType::Start => debug!("EventGenerator {} Start Event", self.id()),
                    EventType::Stop => {
                            debug!("EventGenerator {} Stop Event", self.id());
                            return
                        },
                    _ => ()
                },
                Err(TryRecvError::Empty) => self.process(out_port),
                Err(TryRecvError::Disconnected) => return
            }
            thread::yield_now();
        }
    }
}

impl EventGenerator {
    fn process(&self, out_port: &OutputPort) {
        // TODO
    }
}

struct LoggingSink {
    id: String
}

impl LoggingSink {
    pub fn new(id: String) -> Self {
        debug!("LoggingSink");
        let mut in_ports = HashMap::new();
        in_ports.insert("in".to_string(), InputPort::new());
        LoggingSink {
            id
        }
    }
}

impl Block for LoggingSink {
    fn id(&self) -> &str {
        &self.id
    }

    fn init(&self) {
    }

    fn shutdown(&self) {
    }

    fn thread_executor(&self, ports: BlockPorts) {
        let _in_port = ports.input_ports.get("in").unwrap();
        loop {
            match ports.system_port.receive() {
                Ok(system_event) => match system_event.event_type {
                    EventType::Start => debug!("LoggingSink {} Start Event", self.id()),
                    EventType::Stop => {
                            debug!("LoggingSink {} Stop Event", self.id());
                            return
                        },
                    _ => thread::yield_now()
                },
                Err(TryRecvError::Empty) => thread::yield_now(),
                Err(TryRecvError::Disconnected) => return
            }
        }
    }
}
