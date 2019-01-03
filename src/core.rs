extern crate yaml_rust;

pub struct Event {
}

pub trait Block {
    fn id(&self) -> &str;
    fn init(&self);
    fn shutdown(&self);
}

pub struct Port {
    is_input: bool
}
