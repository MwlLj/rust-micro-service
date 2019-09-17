use crate::register;
use crate::register::IRegister;
use crate::service;
use crate::consts;
use crate::structs;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time;

pub struct CBuffer {
    manager: Arc<Mutex<register::manager::CManager>>,
    serviceItems: Arc<Mutex<HashMap<String, service::CService>>>
}

impl CBuffer {
    pub fn getService(&self, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        let mut serviceItems = match self.serviceItems.lock() {
            Ok(s) => s,
            Err(err) => {
                println!("lock serviceItems error, err: {}", err);
                return None;
            }
        };
        match serviceItems.get_mut(cond.name) {
            Some(s) => {
                s.service(cond)
            },
            None => {
                let mut service = match service::CService::new(cond.name, cond.regCenterType) {
                    Some(s) => s,
                    None => {
                        println!("service new error");
                        return None;
                    }
                };
                let services = match CBuffer::getServicesFromRegisterCenter(self.manager.clone(), cond.name, cond.regCenterType) {
                    Some(s) => s,
                    None => {
                        println!("getServicesFomrRegisterCenter error");
                        return None;
                    }
                };
                service.updateServices(services);
                let info = service.service(cond);
                serviceItems.insert(cond.name.to_string(), service);
                info
            }
        }
    }
}

struct CServiceItem {
    name: String,
    regCenterType: String,
    services: Vec<structs::service::CServiceInfo>
}
impl CBuffer {
    fn syncData(&self, syncIntervalMs: u64) {
        let manager = self.manager.clone();
        let serviceItems = self.serviceItems.clone();
        thread::spawn(move || {
            loop {
                CBuffer::sync(manager.clone(), serviceItems.clone());
                thread::sleep(time::Duration::from_millis(syncIntervalMs));
            }
        });
    }

    fn sync(manager: Arc<Mutex<register::manager::CManager>>, serviceItems: Arc<Mutex<HashMap<String, service::CService>>>) {
        let mut names = Vec::new();
        {
            // avoid occupy mutex
            let mut serviceItems = match serviceItems.lock() {
                Ok(items) => items,
                Err(err) => {
                    println!("lock serviceItemss error, err: {}", err);
                    return;
                }
            };
            for (k, v) in serviceItems.iter_mut() {
                // get service data from register center and update memory
                let mut dbServices = match CBuffer::getServicesFromRegisterCenter(manager.clone(), &k, v.getRegCenterType()) {
                    Some(s) => s,
                    None => {
                        continue;
                    }
                };
                // update service object memory
                v.syncData(&mut dbServices);
                names.push(CServiceItem{
                    name: k.clone(),
                    regCenterType: v.getRegCenterType().to_string(),
                    services: dbServices.clone()
                });
            }
        }
        for item in names {
            CBuffer::updateServicesToRegisterCenter(manager.clone(), &item);
        }
    }

    fn updateServicesToRegisterCenter(manager: Arc<Mutex<register::manager::CManager>>, item: &CServiceItem) {
        let manager = match manager.lock() {
            Ok(m) => m,
            Err(err) => {
                println!("lock manager error, err: {}", err);
                return;
            }
        };
        let register = match manager.get(&item.regCenterType) {
            Some(r) => r,
            None => {
                return;
            }
        };
        register.updateServices(&item.services);
    }

    fn getServicesFromRegisterCenter(manager: Arc<Mutex<register::manager::CManager>>, name: &str, regCenterType: &str) -> Option<Vec<structs::service::CServiceInfo>> {
        let manager = match manager.lock() {
            Ok(m) => m,
            Err(err) => {
                println!("lock manager error, err: {}", err);
                return None;
            }
        };
        let register = match manager.get(&regCenterType) {
            Some(r) => r,
            None => {
                return None;
            }
        };
        register.getServices(name)
    }
}

impl CBuffer {
    pub fn new(centers: &Vec<structs::config::CRegisterCenter>, syncIntervalMs: u64) -> CBuffer {
        let buffer = CBuffer{
            manager: Arc::new(Mutex::new(register::manager::CManager::new(centers))),
            serviceItems: Arc::new(Mutex::new(HashMap::new()))
        };
        buffer.syncData(syncIntervalMs);
        buffer
    }
}
