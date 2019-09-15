use crate::register;
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
    manager: Arc<Mutex<register::manager::CManager>>,
    serviceName: String,
    regCenterType:String,
    select: Box<dyn ISelect + Sync>
}

impl CService {
    pub fn service(&self, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        self.select.service(cond)
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
}

impl CService {
    pub fn new(name: &str, regCenterType: &str, selectType: &str, manager: Arc<Mutex<register::manager::CManager>>) -> Option<CService> {
        if selectType == consts::proto::select_type_random {
            return Some(CService{
                manager: manager,
                serviceName: name.to_string(),
                regCenterType: regCenterType.to_string(),
                select: Box::new(CRandom::new())
            });
        }
        None
    }
}

pub mod random;
