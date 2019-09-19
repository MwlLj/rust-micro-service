use crate::structs;

pub trait ISession {
    // fn new(dial: &str) -> Self;
    fn getValidService(&self, name: &str) -> Option<Vec<structs::service::CService>>;
}

pub mod consul;
pub mod zookeeper;
