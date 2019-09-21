use crate::structs;
use super::ISelect;

use rand::{Rng};

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub struct CMinConnect {
}

impl ISelect for CMinConnect {
    fn service(&mut self, services: &mut Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        let len = services.len();
        if len == 0 {
            println!("services size == 0");
            return None;
        }
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0, len);
        let obj = match services.get_mut(n) {
            Some(o) => o,
            None => {
                println!("not found from services");
                return None;
            }
        };
        // callTimes + 1 for be select service from memory
        obj.callTimes += 1;
        Some(structs::proto::CService{
            serviceId: obj.serviceId.clone(),
            serviceName: obj.serviceName.clone(),
            addr: obj.addr.clone(),
            proto: obj.proto.clone(),
            port: obj.port
        })
    }

    fn isUpdateRegCenter(&self) -> bool {
        true
    }
    
    fn rewrite(&mut self, dbService: &mut structs::service::CServiceInfo, memoryService: &structs::service::CServiceInfo) -> bool {
        if memoryService.callTimes == 0 {
            return false;
        }
        dbService.callTimes += memoryService.callTimes;
        println!("{:?}", dbService.callTimes);
        true
    }
}

impl CMinConnect {
    pub fn new() -> CMinConnect {
        CMinConnect{
        }
    }
}
