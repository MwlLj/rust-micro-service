use crate::structs;

pub trait IProto {
    fn start(&self, service: &structs::service::CServiceRegister, heart: &structs::heart::CHeart) -> Result<(), &str>;
}

pub mod http;
