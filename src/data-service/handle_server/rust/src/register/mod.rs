use consul_client::structs::agent;

pub trait IRegister {
    // fn new(dial: &str) -> Self;
    fn registerHandler(&self, service: &agent::CServiceRegister) -> Result<(), &str>;
}

pub mod consul;
pub mod zookeeper;
