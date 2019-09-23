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

    fn updateMemoryFromLocal(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &mut Vec<structs::service::CServiceInfo>) {
        let min = self.minCallTimesByMemory(dbServices, memoryServices);
        self.redressMemory(min, dbServices, memoryServices);
    }

    fn updateMemory(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &mut Vec<structs::service::CServiceInfo>) {
        let min = self.minCallTimesByDb(dbServices, memoryServices);
        self.redressMemory(min, dbServices, memoryServices);
    }
}

impl CMinConnect {
    fn redressMemory(&self, min: u64, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &mut Vec<structs::service::CServiceInfo>) {
        memoryServices.clear();
        for item in dbServices {
            let mut ss = item.clone();
            if ss.callTimes == 0 {
                ss.callTimes = min;
            }
            // ss.callTimes = 0;
            memoryServices.push(ss);
        }
        println!("{:?}", &memoryServices);
    }

    fn minCallTimesByDb(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &Vec<structs::service::CServiceInfo>) -> u64 {
        let mut services: Vec<&structs::service::CServiceInfo> = Vec::new();
        for item in dbServices {
            if item.callTimes == 0 {
                continue;
            }
            services.push(item);
        }
        // println!("minCallTimes, services: {:?}", services);
        self.minCallTimes(&services)
    }

    fn minCallTimesByMemory(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &Vec<structs::service::CServiceInfo>) -> u64 {
        // find exist dbServices and memoryServices
        // get not include 0 vec
        let mut memoryTmp = HashMap::new();
        for item in memoryServices.iter() {
            memoryTmp.insert(&item.serviceId, item);
        }
        let mut services: Vec<&structs::service::CServiceInfo> = Vec::new();
        for item in dbServices {
            // println!("from memoryMap: {:?} find {}", &memoryTmp, &item.serviceId);
            match memoryTmp.get(&item.serviceId) {
                Some(s) => {
                    if s.callTimes == 0 {
                        // println!("item.callTimes == 0");
                        continue;
                    }
                    services.push(s);
                },
                None => {
                }
            }
        }
        // println!("minCallTimes, services: {:?}", services);
        self.minCallTimes(&services)
    }

    fn minCallTimes(&self, services: &Vec<&structs::service::CServiceInfo>) -> u64 {
        let len = services.len();
        if len == 0 {
            /*
            ** The situation that caused this result:
            */
            return 0;
        }
        let mut minCallTimesIndex = 0;
        let mut minCallTimes = match services.get(minCallTimesIndex) {
            Some(v) => v.callTimes,
            None => {
                return 0;
            }
        };
        for (index, item) in services.iter().enumerate() {
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
