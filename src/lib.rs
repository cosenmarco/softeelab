#[macro_use]
extern crate log;
extern crate simple_logger;

use clap::ArgMatches;
use yaml_rust::{YamlLoader, Yaml};
use std::fs;

mod core;
mod blocks;

pub fn run(matches: ArgMatches) {
    let model_file = matches.value_of("model").unwrap();
    info!("Model file is {}", model_file);

    let model_file_content = fs::read_to_string(model_file)
        .expect("Something went wrong reading the file");

    let docs = YamlLoader::load_from_str(&model_file_content)
        .expect("Error parsing model yaml");
        
    let doc = &docs[0];
    //debug!("{:?}", doc);

    let model = Model::new(doc).unwrap();
    model.run();
    model.stop();
}

pub struct Model {
    blocks: Vec<Box<dyn core::Block>>
}

impl Model {
    fn new(model_doc: &Yaml) -> Result<Self, String> {
        let blocks_vec = model_doc["blocks"].as_vec().unwrap();
        let blocks: Result<Vec<_>, _> = blocks_vec.iter()
            .map(|block_yaml| blocks::build_block(block_yaml))
            .collect();
        match blocks {
            Ok(blocks) => Ok(Model{ 
                    blocks 
                }),
            Err(err) => Err(err)
        }
    }

    fn run(&self) {
        println!("RUN!");
    }

    fn stop(&self) {
        println!("STOP");
    }
}
