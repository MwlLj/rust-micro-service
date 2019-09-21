use crate::structs;
use super::ISelect;

use rand::{Rng};

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub struct CMinConnect {
    callTimes: u64
}

impl ISelect for CMinConnect {
    fn service(&mut self, services: &Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<(structs::proto::CService, structs::service::CServiceInner)> {
        // to do => callTimes + 1
        let len = services.len();
        if len == 0 {
            println!("services size == 0");
            return None;
        }
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0, len);
        let obj = match services.get(n) {
            Some(o) => o,
            None => {
                println!("not found from services");
                return None;
            }
        };
        self.callTimes += 1;
        Some((structs::proto::CService{
            serviceId: obj.serviceId.clone(),
            serviceName: obj.serviceName.clone(),
            addr: obj.addr.clone(),
            proto: obj.proto.clone(),
            port: obj.port
        }, structs::service::CServiceInner{
            callTimes: self.callTimes
        }))
    }

    fn isUpdateRegCenter(&self) -> bool {
        true
    }
    
    fn rewrite(&mut self, dbService: &mut structs::service::CServiceInfo, memoryService: &structs::service::CServiceInfo) {
        dbService.callTimes += self.callTimes;
        println!("{:?}", dbService.callTimes);
        self.callTimes = 0;
    }
}

impl CMinConnect {
    pub fn new() -> CMinConnect {
        CMinConnect{
            callTimes: 0
        }
    }
}
