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
    fn updateMemory(&self, dbServices: &Vec<structs::service::CServiceInfo>, memoryServices: &mut Vec<structs::service::CServiceInfo>);
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
        // println!("CService::service be called");
        let service = match self.selectManager.service(&cond.selectType, &mut self.services, cond) {
            Some(s) => s,
            None => {
                return None;
            }
        };
        self.curRegCenterType = cond.regCenterType.to_string();
        self.curSelectType = cond.selectType.to_string();
        Some(service)
    }

    pub fn isUpdateRegCenter(&self) -> bool {
        self.selectManager.isUpdateRegCenter(&self.curSelectType)
    }

    pub fn initServices(&mut self, services: &Vec<structs::service::CServiceInfo>) {
        self.selectManager.updateMemory(&self.curSelectType, services, &mut self.services);
        /*
        let min = self.minCallTimes(&services);
        for item in services {
            let mut ss = item.clone();
            if ss.callTimes == 0 {
                ss.callTimes = min;
            }
            // ss.callTimes = 0;
            self.services.push(ss);
        }
        */
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

    /*
    ** update memory services by dbServices
    ** dbServices: services from register center
    */
    pub fn syncData(&mut self, dbServices: &mut Vec<structs::service::CServiceInfo>) {
        if self.selectManager.isUpdateRegCenter(&self.curSelectType) {
            println!("mod.rs syncData start, dbServices len: {}, self.services len: {}", dbServices.len(), self.services.len());
            // need update
            let mut memoryMap = HashMap::new();
            for item in &self.services {
                memoryMap.insert(item.serviceId.clone(), item.clone());
            }
            let mut removeIds = Vec::new();
            for item in dbServices.iter_mut() {
                match memoryMap.get_mut(&item.serviceId) {
                    Some(s) => {
                        if !self.selectManager.rewrite(&self.curSelectType, item, s) {
                            removeIds.push(item.serviceId.clone());
                        }
                    },
                    None => {
                    }
                };
            }
            // get min after dbServices's callTimes plus 
            self.selectManager.updateMemory(&self.curSelectType, dbServices, &mut self.services);
            /*
            let min = self.minCallTimes(&dbServices);
            self.services.clear();
            for item in dbServices.iter() {
                let mut ss = item.clone();
                // ss.callTimes = 0;
                if ss.callTimes == 0 {
                    ss.callTimes = min;
                }
                self.services.push(ss);
            }
            */
            // remove doesn't need update services
            for id in removeIds {
                // println!("remove not nedd update, id: {} ...", id);
                match dbServices.iter().position(|x| {
                    if &x.serviceId == &id {
                        true
                    } else {
                        false
                    }
                }) {
                    Some(pos) => {
                        dbServices.remove(pos);
                    },
                    None => {
                    }
                }
            }
            println!("mod.rs syncData end, dbServices len: {}, self.services len: {}", dbServices.len(), self.services.len());
            // println!("dbServices len: {}", dbServices.len());
        } else {
            // doesn't need update
            self.initServices(dbServices);
        }
    }

    pub fn getRegCenterType(&self) -> &str {
        &self.curRegCenterType
    }
}

impl CService {
    /*
    fn minCallTimes(&self, dbServices: &Vec<structs::service::CServiceInfo>) -> u64 {
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
            for item in &self.services {
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
    */

    fn updateServicesWithCheck(&mut self, services: &Vec<structs::service::CServiceInfo>) {
        // iter services
        // check self.services is exists item, if exist -> use exist callTimes, otherwise -> use 0 as callTimes
        let mut tmpMap = HashMap::new();
        for item in &self.services {
            tmpMap.insert(item.serviceId.clone(), item.clone());
        }
        self.services.clear();
        for item in services {
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
            self.services.push(ss);
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
