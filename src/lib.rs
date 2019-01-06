#[macro_use]
extern crate log;
extern crate simple_logger;

#[macro_use]
extern crate serde_derive;

use clap::ArgMatches;
use std::fs;

mod model;
use crate::model::*;

use std::thread;

pub fn run(matches: ArgMatches) {
    let model_file = matches.value_of("model").unwrap();
    info!("Model file is {}", model_file);

    let model_file_content = fs::read_to_string(model_file)
        .expect("Something went wrong reading the file");

    debug!("YAML content is: {:?}", model_file_content);

    let deserialized_model_def: ModelDef = serde_yaml::from_str(&model_file_content).unwrap();
    debug!("{:?}", deserialized_model_def);

    let mut model = Model::new(deserialized_model_def).unwrap();
    model.run().unwrap();
    thread::sleep_ms(10000);
    model.stop();
}

