pub mod core;
pub mod blocks;

use self::core::*;
use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::sync::{Mutex, Arc};

/// Here the basic building blocks of the Model

pub struct Model {
    blocks: HashMap<String, BoxedBlock>,
    connections: Vec<ModelDefConnection>,
    system_channels: Vec<Sender<Event>>,
    threads: Vec<JoinHandle<()>>
}

impl Model {
    pub fn new(model_def: ModelDef) -> Result<Self, String> {
        let mut blocks: HashMap<String, BoxedBlock> = HashMap::new();
        for block_def in model_def.blocks {
            let block = blocks::build_block(block_def)?;
            blocks.insert(block.id().to_string(), block);
        }
        let connections = model_def.connections;
        Ok(Model {
            blocks,
            connections,
            system_channels: Vec::new(),
            threads: Vec::new()
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("RUN!");
        self.create_channels()?;
        self.init_blocks();
        self.start_threads();
        self.send_start_events();
        Ok(())
    }

    pub fn stop(&mut self) {
        println!("STOP");
        self.send_stop_events();
        self.join_threads();
        self.close_system_channels();
        self.shudown_blocks();
    }

    fn create_channels(&mut self) -> Result<(), String> {
        // for connection in &mut self.connections {
        //     for destination in &mut connection.to {
        //         let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        //         let (from_block, to_block) = self.get_blocks(&connection.from.block, &destination.block)?;
        //         let (from_port, to_port) = Model::get_ports(from_block, &connection.from.port, 
        //             to_block, &destination.port)?;
        //         from_port.add_sender(sender);
        //         to_port.set_receiver(receiver);
        //     }
        // }
        // for block in &mut self.blocks {
        //     let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        //     block.1.system_port().set_receiver(receiver);
        //     self.system_channels.push(sender);
        // }
        Ok(())
    }

    // fn get_blocks(&self, from_id: &str, to_id: &str) 
    //             -> Result<(&BoxedBlock, &BoxedBlock), String> {
    //     if let Some(from_block) = self.blocks.get(from_id) {
    //         if let Some(to_block) = self.blocks.get(to_id) {
    //             Ok((from_block, to_block))
    //         } else {
    //             Err(format!("Cannot find block {}", to_id))
    //         }
    //     } else {
    //         Err(format!("Cannot find block {}", from_id))
    //     }
    // }

    // fn get_ports<'a, 'b>(from: &'a BoxedBlock, from_port_id: &str, to: &'b BoxedBlock, to_port_id: &str) 
    //             -> Result<(&'a OutputPort, &'b InputPort), String> {
    //     let from_port = from.output_ports().get(from_port_id);
    //     let to_port = to.input_ports().get(to_port_id);
    //     if let None = from_port {
    //         Err(format!("Cannot find port {} in block {}", from_port_id, from.id()))
    //     } else if let None = from_port {
    //         return Err(format!("Cannot find port {} in block {}", from_port_id, from.id()));
    //     } else {
    //         Ok((from_port.unwrap(), to_port.unwrap()))
    //     }
    // }

    fn init_blocks(&self) {
        for block in &self.blocks {
            block.1.init();
        }
    }

    fn start_threads(&mut self) {
    }

    fn send_start_events(&self) {
    }

    fn send_stop_events(&self) {
    }

    fn close_system_channels(&mut self) {
        self.system_channels = Vec::new();
    }

    fn join_threads(&self) {
    }

    fn shudown_blocks(&self) {
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
    //pub implementation: String,
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