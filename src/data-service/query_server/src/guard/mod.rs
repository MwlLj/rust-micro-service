use crate::structs;

use consul_client::structs::agent;

pub trait IGuard {
    // fn new(dial: &str) -> Self;
    fn registerQueryer(&self, service: &agent::CServiceRegister) -> Result<(), &str>;
}

pub mod consul;
pub mod zookeeper;
