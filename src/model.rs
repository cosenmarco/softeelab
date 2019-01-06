pub mod core;
pub mod blocks;

use self::core::*;
use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::sync::{Mutex, Arc};
use time;


/// Here the basic building blocks of the Model

pub struct Model {
    blocks: Vec<ModelDefBlock>,
    connections: Vec<ModelDefConnection>,
    system_channels: Vec<Sender<Event>>,
    threads: Vec<JoinHandle<()>>
}

impl Model {
    pub fn new(model_def: ModelDef) -> Result<Self, String> {
        let blocks = model_def.blocks;
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
        let mut blocks = self.create_blocks();
        let mut ports = self.create_ports(blocks.keys().collect())?;
        Model::init_blocks(&mut blocks);
        self.start_threads(&mut blocks, &mut ports);
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

    fn create_blocks(&self) -> HashMap<String, BoxedBlock> {
        self.blocks.iter()
            .map(|block_def| {
                let block = blocks::build_block(block_def);
                (block.id().to_owned(), block)
            })
            .collect()
    }

    fn init_blocks(blocks: &mut HashMap<String, BoxedBlock>) {
        for block in blocks.values() {
            block.init();
        }
    }

    fn create_ports(&mut self, block_ids: Vec<&String>) -> Result<HashMap<String, BlockPorts>, String> {
        let mut result: HashMap<String, BlockPorts> = HashMap::new();
        for block in block_ids {
            result.insert(block.to_owned(), BlockPorts{
                system_port: InputPort::new(),
                input_ports: HashMap::new(),
                output_ports: HashMap::new()
            });
        }
        for connection in &self.connections {
            for destination in &connection.to {
                let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();

                {
                    let from_block_ports = result.get_mut(&connection.from.block)
                        .ok_or(format!("Cannot find block {} to build connection from {:?}",
                                &connection.from.block, connection.from))?;

                    let from_port = from_block_ports.output_ports
                        .entry(connection.from.port.to_owned())
                        .or_insert(OutputPort::new());

                    from_port.add_sender(sender);
                }

                {
                    let to_block_ports = result.get_mut(&destination.block)
                        .ok_or(format!("Cannot find block {} to build connection to {:?}",
                            &connection.from.block, destination))?;

                    let to_port = to_block_ports.input_ports
                        .entry(destination.port.to_owned())
                        .or_insert(InputPort::new());

                    to_port.set_receiver(receiver);
                }
            }
        }
        Ok(result)
    }

    fn start_threads(&mut self, all_blocks: &mut HashMap<String, BoxedBlock>, all_ports: &mut HashMap<String, BlockPorts>) {
        for block_entry in all_blocks.drain() {
            let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
            self.system_channels.push(sender);
            let mut ports = all_ports.remove(&block_entry.0).unwrap();
            let mut block = block_entry.1;
            ports.system_port.set_receiver(receiver);
            self.threads.push(thread::spawn(move || {
                debug!("Start thread for {}", block.id());
                block.thread_executor(ports);
            }));
        }
    }

    fn send_start_events(&mut self) {
        debug!("Sending start events for {} channels", self.system_channels.len());
        let start = time::precise_time_ns();
        for system_channel in self.system_channels.iter_mut() {
            system_channel.send(Event::new(start, EventType::Start)).unwrap();
        }
    }

    fn send_stop_events(&mut self) {
        let stop = time::precise_time_ns();
        for system_channel in self.system_channels.iter_mut() {
            system_channel.send(Event::new(stop, EventType::Stop)).unwrap();
        }
    }

    fn close_system_channels(&mut self) {
        self.system_channels = Vec::new();
    }

    fn join_threads(&mut self) {
        for thread in self.threads.drain(..) {
            thread.join().unwrap();
        }
    }

    fn shudown_blocks(&self) {
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModelDef {
    pub blocks: Vec<ModelDefBlock>,
    pub connections: Vec<ModelDefConnection>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModelDefConnection {
    pub from: ModelDefConnectionPort,
    pub to: Vec<ModelDefConnectionPort>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModelDefConnectionPort {
    pub block: String,
    pub port: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModelDefBlock {
    pub id: String,
    //pub implementation: String,
    #[serde(flatten)]
    pub configuration: ModelDefBlockConfig
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "implementation", content = "configuration")]
pub enum ModelDefBlockConfig {
    EventGenerator(ModelDefEventGeneratorConfiguration),
    LoggingSink
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModelDefEventGeneratorConfiguration {
    pub event_type: String,
    pub frequency: f64
}