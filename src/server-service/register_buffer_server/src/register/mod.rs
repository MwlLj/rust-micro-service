use crate::structs;

use consul_client::structs::agent;

use std::collections::HashMap;

pub trait IRegister {
    fn getServices(&self, name: &str) -> Option<Vec<structs::service::CServiceInfo>>;
    fn addService(&self, service: &agent::CServiceRegister) -> Result<(), &str>;
    fn updateServices(&self, name: &str, memoryServices: &HashMap<String, structs::service::CServiceInfo>);
}

pub mod consul;
pub mod zookeeper;
pub mod manager;