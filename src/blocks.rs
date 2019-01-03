use crate::core::Block;
use crate::core::Port;
use yaml_rust::{Yaml, yaml::Hash};

use std::collections::HashMap;

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
    ports: HashMap<String, Port>
}

impl EventGenerator {
    pub fn new(id: String, config: Option<&Hash>) -> Self {
        let ports = HashMap::new();
        EventGenerator {
            id,
            ports
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
}


struct LoggingSink {
    id: String,
    ports: HashMap<String, Port>
}

impl LoggingSink {
    pub fn new(id: String, config: Option<&Hash>) -> Self {
        let ports = HashMap::new();
        LoggingSink {
            id,
            ports
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
}
