use crate::structs;
use super::ISelect;

use rand::{Rng};

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub struct CMinConnect {
}

impl ISelect for CMinConnect {
    fn service(&mut self, services: &mut Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        // println!("minconnect .., {:?}", &services);
        // get callTimes min service
        let len = services.len();
        if len == 0 {
            println!("services size == 0");
            return None;
        }
        let mut minCallTimesIndex = 0;
        let mut minCallTimes = match services.get(minCallTimesIndex) {
            Some(v) => v.callTimes,
            None => {
                return None;
            }
        };
        for (index, item) in services.iter().enumerate() {
            if item.callTimes < minCallTimes {
                minCallTimes = item.callTimes;
                minCallTimesIndex = index;
            }
        }
        let obj = match services.get_mut(minCallTimesIndex) {
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
        /*
        if memoryService.callTimes == 0 {
            return false;
        }
        dbService.callTimes += memoryService.callTimes;
        println!("{:?}", dbService.callTimes);
        true
        */
        // println!("mc: {}, dc: {}", memoryService.callTimes, dbService.callTimes);
        if memoryService.callTimes <= dbService.callTimes {
            return false;
        }
        dbService.callTimes += (memoryService.callTimes - dbService.callTimes);
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
