use crate::consts;
use crate::structs;
use crate::proto;

pub trait IProto {
    fn start(&self, service: &structs::service::CServiceRegister, heart: &structs::heart::CHeart) -> Result<(), &str>;
}

pub fn start(config: structs::config::CConfigInfo) {
    if config.start.listen.proto == consts::proto::proto_type_http {
        let proto = match proto::http::CHttp::new(&config.start) {
            Some(q) => q,
            None => {
                println!("proto http new error");
                return;
            }
        };
        proto.start(&structs::service::CServiceRegister{
            serviceId: config.register.serviceId,
            serviceName: config.register.serviceName
        }, &config.heart);
    }
}

pub mod http;
