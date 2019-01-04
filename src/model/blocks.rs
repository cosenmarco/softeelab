use super::core::*;
use yaml_rust::{Yaml, yaml::Hash};

use std::collections::HashMap;

/// Here the Block implementations and a factory to build the various blocks
/// starting from configuration

pub fn build_block(block_yaml: &Yaml) -> Result<Box<dyn Block>, String> {
    debug!("{:?}", block_yaml);
    let config_yaml = block_yaml["config"].as_hash();
    let block_id = match block_yaml["id"].as_str() {
        Some(string) => string.to_owned(),
        None => return Err("Every block must have an id".to_string())
    };
    match block_yaml["impl"].as_str() {
        Some("EventGenerator") => Ok(Box::new(
                EventGenerator::new(block_id, config_yaml)
            )),
        Some("LoggingSink") => Ok(Box::new(
                LoggingSink::new(block_id, config_yaml)
            )),
        Some(implem) => Err(format!("Unknown impl {} for block {}", implem, block_id)),
        None => Err(format!("Must specify an implementation for block {}", block_id))
    }
}

struct EventGenerator {
    id: String,
    in_ports: HashMap<String, InputPort>,
    out_ports: HashMap<String, OutputPort>
}

impl EventGenerator {
    pub fn new(id: String, config: Option<&Hash>) -> Self {
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
    pub fn new(id: String, config: Option<&Hash>) -> Self {
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
