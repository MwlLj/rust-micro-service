use crate::structs;
use crate::consts;
use manager::CManager;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub trait ISelect {
    fn service(&mut self, services: &mut Vec<structs::service::CServiceInfo>, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService>;
    /*
    ** return: if need change to register center is true, otherwise is false
    */
    fn rewrite(&mut self, dbService: &mut structs::service::CServiceInfo, memoryService: &structs::service::CServiceInfo) -> bool;
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
        let service = match self.selectManager.service(&cond.selectType, &mut self.services, cond) {
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

    pub fn initServices(&mut self, services: Vec<structs::service::CServiceInfo>) {
        for item in services {
            let mut ss = item.clone();
            ss.callTimes = 0;
            self.services.push(ss);
        }
    }

    pub fn clearServices(&mut self) {
        self.services.clear();
    }

    pub fn getServices(&self) -> HashMap<String, structs::service::CServiceInfo> {
        let mut services = HashMap::new();
        for item in &self.services {
            services.insert(item.serviceId.to_string(), item.clone());
        }
        services
    }

    pub fn syncData(&mut self, dbServices: &mut Vec<structs::service::CServiceInfo>) {
        if self.selectManager.isUpdateRegCenter(&self.curSelectType) {
            // need update
            let mut memoryMap = HashMap::new();
            for item in &self.services {
                memoryMap.insert(item.serviceId.clone(), item.clone());
            }
            self.services.clear();
            let mut removeIndex = Vec::new();
            for (index, item) in dbServices.iter_mut().enumerate() {
                let s = match memoryMap.get_mut(&item.serviceId) {
                    Some(s) => s,
                    None => {
                        continue;
                    }
                };
                if !self.selectManager.rewrite(&self.curSelectType, item, s) {
                    let mut ss = item.clone();
                    ss.callTimes = 0;
                    self.services.push(ss);
                    removeIndex.push(index);
                } else {
                    let mut ss = item.clone();
                    ss.callTimes = 0;
                    self.services.push(ss);
                }
            }
            // remove doesn't need update services
            for index in removeIndex {
                println!("remove not nedd update, index: {} ...", index);
                dbServices.remove(index);
            }
        } else {
            // doesn't need update
            self.services.clear();
            for item in dbServices {
                let mut ss = item.clone();
                ss.callTimes = 0;
                self.services.push(ss);
            }
        }
        println!("self.services len: {}", self.services.len());
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
            curRegCenterType: consts::proto::register_center_type_consul.to_string(),
            curSelectType: consts::proto::select_type_random.to_string(),
            services: Vec::new(),
            selectManager: CManager::new()
        });
    }
}

pub mod random;
pub mod min_connect;
pub mod manager;
