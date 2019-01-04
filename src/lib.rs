#[macro_use]
extern crate log;
extern crate simple_logger;

use clap::ArgMatches;
use yaml_rust::YamlLoader;
use std::fs;

mod model;
use crate::model::Model;

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

