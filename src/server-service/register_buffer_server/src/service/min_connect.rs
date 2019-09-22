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
        println!("mc: {}, dc: {}", memoryService.callTimes, dbService.callTimes);
        if memoryService.callTimes <= dbService.callTimes {
            return false;
        }
        dbService.callTimes += (memoryService.callTimes - dbService.callTimes);
        println!("{:?}", dbService.callTimes);
        true
    }

    fn updateMemory(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &mut Vec<structs::service::CServiceInfo>) {
        let min = self.minCallTimes(dbServices, memoryServices);
        println!("min: {}", min);
        memoryServices.clear();
        for item in dbServices {
            let mut ss = item.clone();
            if ss.callTimes == 0 {
                ss.callTimes = min;
            }
            // ss.callTimes = 0;
            memoryServices.push(ss);
        }
    }
}

impl CMinConnect {
    fn minCallTimes(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &Vec<structs::service::CServiceInfo>) -> u64 {
        // get not include 0 vec
        let mut ss = Vec::new();
        for item in dbServices.iter() {
            if item.callTimes == 0 {
                continue;
            }
            ss.push(item);
        }
        let len = ss.len();
        if len == 0 {
            // dbServices all is 0 -> get from memory
            /*
            ** The situation that caused this result:
            ** if doesn't need update to register center
            */
            for item in memoryServices {
                if item.callTimes == 0 {
                    continue;
                }
                ss.push(item);
            }
            if ss.len() == 0 {
                /*
                ** dbServices and memory both 0
                ** The situation that caused this result:
                ** vec is empty
                */
                return 0;
            }
        }
        let mut minCallTimesIndex = 0;
        let mut minCallTimes = match ss.get(minCallTimesIndex) {
            Some(v) => v.callTimes,
            None => {
                return 0;
            }
        };
        for (index, item) in ss.iter().enumerate() {
            if item.callTimes < minCallTimes {
                minCallTimes = item.callTimes;
                minCallTimesIndex = index;
            }
        }
        minCallTimes
    }
}

impl CMinConnect {
    pub fn new() -> CMinConnect {
        CMinConnect{
        }
    }
}
