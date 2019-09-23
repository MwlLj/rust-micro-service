use crate::structs;
use super::ISelect;

use rand::{Rng};

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub struct CRandom {
}

impl ISelect for CRandom {
    fn service(&mut self, services: &mut Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
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
        Some(structs::proto::CService{
            serviceId: obj.serviceId.clone(),
            serviceName: obj.serviceName.clone(),
            addr: obj.addr.clone(),
            proto: obj.proto.clone(),
            port: obj.port
        })
    }

    fn isUpdateRegCenter(&self) -> bool {
        false
    }

    fn rewrite(&mut self, dbService: &mut structs::service::CServiceInfo, memoryService: &structs::service::CServiceInfo) -> bool {
        false
    }

    fn updateMemoryFromLocal(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &mut Vec<structs::service::CServiceInfo>) {
        self.updateMemory(dbServices, memoryServices);
    }

    fn updateMemory(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &mut Vec<structs::service::CServiceInfo>) {
        /*
        for item in dbServices {
            let mut ss = item.clone();
            // ss.callTimes = 0;
            memoryServices.push(ss);
        }
        */
        let mut tmpMap = HashMap::new();
        for item in memoryServices.iter() {
            tmpMap.insert(item.serviceId.clone(), item.clone());
        }
        memoryServices.clear();
        for item in dbServices {
            let callTimes = match tmpMap.get(&item.serviceId) {
                Some(s) => {
                    s.callTimes
                },
                None => {
                    0
                }
            };
            let mut ss = item.clone();
            ss.callTimes = callTimes;
            memoryServices.push(ss);
        }
    }
}

impl CRandom {
    pub fn new() -> CRandom {
        CRandom{
        }
    }
}
