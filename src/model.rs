pub mod core;
pub mod blocks;

use self::core::Block;
use std::collections::HashMap;

/// Here the basic building blocks of the Model

pub struct Model {
    blocks: HashMap<String, Box<dyn Block>>
}

impl Model {
    pub fn new(model_def: ModelDef) -> Result<Self, String> {
        let mut blocks: HashMap<String, Box<dyn Block>> = HashMap::new();
        for block_def in model_def.blocks {
            let block = blocks::build_block(block_def)?;
            blocks.insert(block.id().to_string(), block);
        }
        Ok(Model {
            blocks
        })
    }

    pub fn run(&self) {
        println!("RUN!");
    }

    pub fn stop(&self) {
        println!("STOP");
    }
}

#[derive(Deserialize, Debug)]
pub struct ModelDef {
    pub blocks: Vec<ModelDefBlock>,
    pub connections: Vec<ModelDefConnection>
}

#[derive(Deserialize, Debug)]
pub struct ModelDefConnection {
    pub from: ModelDefConnectionPort,
    pub to: Vec<ModelDefConnectionPort>
}

#[derive(Deserialize, Debug)]
pub struct ModelDefConnectionPort {
    pub block: String,
    pub port: String
}

#[derive(Deserialize, Debug)]
pub struct ModelDefBlock {
    pub id: String,
    pub implementation: String,
    #[serde(flatten)]
    pub configuration: ModelDefBlockConfig
}

#[derive(Deserialize, Debug)]
#[serde(tag = "implementation", content = "configuration")]
pub enum ModelDefBlockConfig {
    EventGenerator(ModelDefEventGeneratorConfiguration),
    LoggingSink
}

#[derive(Deserialize, Debug)]
pub struct ModelDefEventGeneratorConfiguration {
    pub event_type: String,
    pub frequency: f64
}