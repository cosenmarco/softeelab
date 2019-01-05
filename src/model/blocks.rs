use super::core::*;
use super::*;

use std::collections::HashMap;

/// Here the Block implementations and a factory to build the various blocks
/// starting from configuration

pub fn build_block(block_def: ModelDefBlock) -> Result<Box<dyn Block>, String> {
    debug!("{:?}", block_def);
    let id = block_def.id;
    match block_def.implementation.as_ref() {
        "EventGenerator" => match block_def.configuration {
            ModelDefBlockConfig::EventGenerator(config) => Ok(Box::new(
                EventGenerator::new(id, config)
            )),
            _ => Err("Expected EventGenerator config".to_string())
        },
        "LoggingSink" => Ok(Box::new(LoggingSink::new(id))),
        implem => Err(format!("Unknown impl {} for block {}", implem, id))
    }
}

struct EventGenerator {
    id: String,
    in_ports: HashMap<String, InputPort>,
    out_ports: HashMap<String, OutputPort>
}

impl EventGenerator {
    pub fn new(id: String, config: ModelDefEventGeneratorConfiguration) -> Self {
        debug!("EventGenerator: {:?}", config);
        let mut out_ports = HashMap::new();
        out_ports.insert("out".to_string(), OutputPort::new());
        EventGenerator {
            id,
            in_ports: HashMap::new(),
            out_ports
        }
    }
}

impl Block for EventGenerator {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn input_ports(&self) -> &HashMap<String, InputPort> {
        &self.in_ports
    }

    fn output_ports(&self) -> &HashMap<String, OutputPort> {
        &self.out_ports
    }

    fn init(&self) {

    }
    fn shutdown(&self) {

    }
}

struct LoggingSink {
    id: String,
    in_ports: HashMap<String, InputPort>,
    out_ports: HashMap<String, OutputPort>
}

impl LoggingSink {
    pub fn new(id: String) -> Self {
        debug!("LoggingSink");
        let mut in_ports = HashMap::new();
        in_ports.insert("in".to_string(), InputPort::new());
        LoggingSink {
            id,
            in_ports,
            out_ports: HashMap::new()
        }
    }
}

impl Block for LoggingSink {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn input_ports(&self) -> &HashMap<String, InputPort> {
        &self.in_ports
    }

    fn output_ports(&self) -> &HashMap<String, OutputPort> {
        &self.out_ports
    }

    fn init(&self) {

    }
    fn shutdown(&self) {

    }
}
