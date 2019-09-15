use crate::structs;
use super::ISelect;

use rand::{Rng};

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub struct CRandom {
    services: Vec<structs::service::CServiceInfo>
}

impl ISelect for CRandom {
    fn getServices(&self) -> &Vec<structs::service::CServiceInfo> {
        &self.services
    }

    fn updateServices(&mut self, services: Vec<structs::service::CServiceInfo>) {
        self.services = services;
    }

    fn service(&self, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        let len = self.services.len();
        if len == 0 {
            println!("services size == 0");
            return None;
        }
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0, len);
        let obj = match self.services.get(n) {
            Some(o) => o,
            None => {
                println!("not found from services");
                return None;
            }
        };
        Some(structs::proto::CService{
            serviceId: obj.serviceId.clone(),
            serviceName: obj.serviceName.clone(),
            addr: obj.addr.clone(),
            proto: obj.proto.clone(),
            port: obj.port
        })
    }
}

impl CRandom {
    pub fn new() -> CRandom {
        CRandom{
            services: Vec::new()
        }
    }
}
