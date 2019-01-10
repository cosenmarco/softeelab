use super::core::*;
use super::*;

use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;
use time;

/// Here the Block implementations and a factory to build the various blocks
/// starting from configuration

pub fn build_block(block_def: &ModelDefBlock) -> BoxedBlock {
    debug!("{:?}", block_def);
    let id = block_def.id.to_owned();
    match &block_def.configuration {
        ModelDefBlockConfig::EventGenerator(config) => 
            Box::new(EventGenerator::new(id, config.clone())),
        ModelDefBlockConfig::LoggingSink => Box::new(LoggingSink::new(id))
    }
}

struct EventGenerator {
    id: String,
    initial_instant: u64,
    previous_instant: u64,
    config: ModelDefEventGeneratorConfiguration,
    period_ns: u64
}

impl EventGenerator {
    pub fn new(id: String, config: ModelDefEventGeneratorConfiguration) -> Self {
        debug!("EventGenerator: {:?}", config);
        let frequency = config.frequency;
        EventGenerator {
            id,
            initial_instant: 0,
            previous_instant: 0,
            config,
            period_ns: (1_000_000_000.0 / frequency) as u64
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

    fn thread_executor(&mut self, ports: BlockPorts) {
        let out_port = ports.output_ports.get("out").unwrap();
        loop {
            match ports.system_port.receive() {
                Ok(system_event) => match system_event {
                    SystemEvent::Start(timestamp) => {
                        debug!("EventGenerator {} Start Event", self.id());
                        self.initial_instant = timestamp;
                        self.previous_instant = timestamp;
                    },
                    SystemEvent::Stop => {
                        debug!("EventGenerator {} Stop Event", self.id());
                        return
                    }
                },
                Err(TryRecvError::Empty) => self.process(out_port),
                Err(TryRecvError::Disconnected) => return
            }
            thread::yield_now();
        }
    }
}

impl EventGenerator {
    fn process(&mut self, out_port: &OutputPort) {
        let now = time::precise_time_ns();
        if now - self.previous_instant >= self.period_ns {
            out_port.send(Event::new(now - self.initial_instant, self.get_event_type()));
            self.previous_instant += self.period_ns;
        }
        thread::yield_now();
    }

    fn get_event_type(&self) -> EventType {
        match self.config.event_type.as_ref() {
            "Trigger" => EventType::Trigger,
            _ => EventType::Trigger
        }
    }
}

struct LoggingSink {
    id: String
}

impl LoggingSink {
    pub fn new(id: String) -> Self {
        debug!("LoggingSink");
        LoggingSink {
            id
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

    fn thread_executor(&mut self, ports: BlockPorts) {
        let in_port = ports.input_ports.get("in").unwrap();
        loop {
            match ports.system_port.receive() {
                Ok(system_event) => match system_event {
                    SystemEvent::Start(_) => debug!("LoggingSink {} Start Event", self.id()),
                    SystemEvent::Stop => {
                            debug!("LoggingSink {} Stop Event", self.id());
                            return
                        }
                },
                Err(TryRecvError::Empty) => self.process(in_port),
                Err(TryRecvError::Disconnected) => return
            }
        }
    }
}

impl LoggingSink {
    fn process(&self, in_port: &InputPort<Event>) {
        if let Ok(event) = in_port.receive() {
            info!("Logging event {:?} in sink {}", event, self.id());
        }
        thread::yield_now();
    }
}
