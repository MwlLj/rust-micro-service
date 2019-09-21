use crate::structs;
use crate::consts;
use manager::CManager;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub trait ISelect {
    fn service(&mut self, services: &Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<(structs::proto::CService, structs::service::CServiceInner)>;
    fn rewrite(&mut self, dbService: &mut structs::service::CServiceInfo, memoryService: &structs::service::CServiceInfo);
    fn isUpdateRegCenter(&self) -> bool;
}

pub struct CService {
    serviceName: String,
    curRegCenterType: String,
    curSelectType: String,
    services: Vec<structs::service::CServiceInfo>,
    selectManager: CManager
}

impl CService {
    pub fn service(&mut self, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        // let select = match self.selectManager.get(&cond.selectType) {
        //     Some(s) => s,
        //     None => {
        //         return None;
        //     }
        // };
        println!("CService::service be called");
        let (service, inner) = match self.selectManager.service(&cond.selectType, &self.services, cond) {
            Some(s) => s,
            None => {
                return None;
            }
        };
        self.curRegCenterType = cond.regCenterType.to_string();
        self.curSelectType = cond.selectType.to_string();
        // self.updateService(&service, &inner);
        Some(service)
    }

    pub fn isUpdateRegCenter(&self) -> bool {
        self.selectManager.isUpdateRegCenter(&self.curSelectType)
    }

    pub fn clearServices(&mut self) {
        self.services.clear();
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
        if self.selectManager.isUpdateRegCenter(&self.curSelectType) {
            for item in dbServices.iter_mut() {
                let s = match memoryMap.get(&item.serviceId) {
                    Some(s) => s,
                    None => {
                        continue;
                    }
                };
                self.selectManager.rewrite(&self.curSelectType, item, s);
            }
        }
        // update memory
        self.updateServices(dbServices.clone());
    }

    pub fn getRegCenterType(&self) -> &str {
        &self.curRegCenterType
    }
}

impl CService {
    fn updateService(&mut self, service: &structs::proto::CService, inner: &structs::service::CServiceInner) {
        for item in self.services.iter_mut() {
            println!("item.serviceId: {}, service.serviceId: {}, inner.callTimes: {}", &item.serviceId, &service.serviceId, inner.callTimes);
            if item.serviceId == service.serviceId {
                item.copyFromInner(service, inner);
                // println!("item.callTimes: {}", item.callTimes);
                break;
            }
        }
    }
}

impl CService {
    pub fn new(name: &str) -> Option<CService> {
        return Some(CService{
            serviceName: name.to_string(),
            curRegCenterType: String::new(),
            curSelectType: String::new(),
            services: Vec::new(),
            selectManager: CManager::new()
        });
    }
}

pub mod random;
pub mod min_connect;
pub mod manager;
