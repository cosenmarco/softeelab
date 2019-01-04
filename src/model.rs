pub mod core;
pub mod blocks;

use self::core::Block;
use yaml_rust::Yaml;
use std::collections::HashMap;

/// Here the basic building blocks of the Model

pub struct Model {
    blocks: HashMap<String, Box<dyn Block>>
}

impl Model {
    pub fn new(model_doc: &Yaml) -> Result<Self, String> {
        let blocks_vec = model_doc["blocks"].as_vec().unwrap();
        let blocks: Result<Vec<_>, _> = blocks_vec.iter()
            .map(|block_yaml| blocks::build_block(block_yaml))
            .collect();

        if let Err(err) = blocks { return Err(err); }

        let mut blocks_map: HashMap<String, Box<dyn Block>> = HashMap::new();
        for block in blocks.unwrap() {
            blocks_map.insert(block.id().to_string(), block);
        }
        Ok(Model {
            blocks: blocks_map
        })
    }

    pub fn run(&self) {
        println!("RUN!");
    }

    pub fn stop(&self) {
        println!("STOP");
    }
}
