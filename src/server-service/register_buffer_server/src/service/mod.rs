use crate::structs;
use crate::consts;
use random::CRandom;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub trait ISelect {
    fn getServices(&self) -> &Vec<structs::service::CServiceInfo>;
    fn updateServices(&mut self, services: Vec<structs::service::CServiceInfo>);
    fn service(&self, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService>;
}

pub struct CService {
    serviceName: String,
    regCenterType: String,
    select: Box<dyn ISelect + Sync + Send + 'static>
}

impl CService {
    pub fn service(&self, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        self.select.service(cond)
    }

    pub fn updateServices(&mut self, services: Vec<structs::service::CServiceInfo>) {
        self.select.updateServices(services);
    }

    pub fn getServices(&self) -> HashMap<String, structs::service::CServiceInfo> {
        let mut services = HashMap::new();
        for item in self.select.getServices() {
            services.insert(item.serviceId.to_string(), item.clone());
        }
        services
    }

    pub fn syncData(&mut self, dbServices: &mut Vec<structs::service::CServiceInfo>) {
        let mut memoryMap = HashMap::new();
        for item in self.select.getServices() {
            memoryMap.insert(item.serviceId.clone(), item.clone());
        }
        // dbData + memoryData
        for item in dbServices.iter_mut() {
            let s = match memoryMap.get(&item.serviceId) {
                Some(s) => s,
                None => {
                    continue;
                }
            };
            item.callTimes += s.callTimes;
        }
        // update memory
        self.select.updateServices(dbServices.clone());
    }

    pub fn getRegCenterType(&self) -> &str {
        &self.regCenterType
    }
}

impl CService {
    pub fn new(name: &str, regCenterType: &str, selectType: &str) -> Option<CService> {
        if selectType == consts::proto::select_type_random {
            return Some(CService{
                serviceName: name.to_string(),
                regCenterType: regCenterType.to_string(),
                select: Box::new(CRandom::new())
            });
        }
        None
    }
}

pub mod random;
