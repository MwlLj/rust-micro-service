use crate::structs;
use crate::consts;
use manager::CManager;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub trait ISelect {
    fn service(&self, services: &Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<(structs::proto::CService, structs::service::CServiceInner)>;
}

pub struct CService {
    serviceName: String,
    regCenterType: String,
    services: Vec<structs::service::CServiceInfo>,
    selectManager: CManager
}

impl CService {
    pub fn service(&mut self, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        let select = match self.selectManager.get(&cond.selectType) {
            Some(s) => s,
            None => {
                return None;
            }
        };
        let (service, inner) = match select.service(&self.services, cond) {
            Some(s) => s,
            None => {
                return None;
            }
        };
        // to do => callTimes + 1
        self.updateService(&service, &inner);
        Some(service)
    }

    pub fn updateServices(&mut self, services: Vec<structs::service::CServiceInfo>) {
        self.services = services;
    }

    pub fn getServices(&self) -> HashMap<String, structs::service::CServiceInfo> {
        let mut services = HashMap::new();
        for item in &self.services {
            services.insert(item.serviceId.to_string(), item.clone());
        }
        services
    }

    pub fn syncData(&mut self, dbServices: &mut Vec<structs::service::CServiceInfo>) {
        let mut memoryMap = HashMap::new();
        for item in &self.services {
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
        self.updateServices(dbServices.clone());
    }

    pub fn getRegCenterType(&self) -> &str {
        &self.regCenterType
    }
}

impl CService {
    fn updateService(&mut self, service: &structs::proto::CService, inner: &structs::service::CServiceInner) {
        for item in self.services.iter_mut() {
            if item.serviceId == service.serviceId {
                item.copyFromInner(service, inner);
                break;
            }
        }
    }
}

impl CService {
    pub fn new(name: &str, regCenterType: &str) -> Option<CService> {
        return Some(CService{
            serviceName: name.to_string(),
            regCenterType: regCenterType.to_string(),
            services: Vec::new(),
            selectManager: CManager::new()
        });
    }
}

pub mod random;
pub mod manager;
