use crate::structs;

pub trait IQuery {
    fn start(&self, service: &structs::service::CServiceRegister, heart: &structs::heart::CHeart) -> Result<(), &str>;
}

pub mod http;
