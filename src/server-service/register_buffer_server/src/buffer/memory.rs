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
    pub fn getService(&self, regCenterType: &str, cond: &structs::buffer::CServiceQueryCond) -> Option<structs::proto::CService> {
        let mut serviceItems = match self.serviceItems.lock() {
            Ok(s) => s,
            Err(err) => {
                println!("lock serviceItems error, err: {}", err);
                return None;
            }
        };
        match serviceItems.get(cond.name) {
            Some(s) => {
                s.service(cond)
            },
            None => {
                let mut service = match service::CService::new(cond.name, regCenterType, cond.selectType) {
                    Some(s) => s,
                    None => {
                        println!("service new error");
                        return None;
                    }
                };
                let services = match CBuffer::getServicesFromRegisterCenter(self.manager.clone(), cond.name, regCenterType) {
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
    services: HashMap<String, structs::service::CServiceInfo>
}
impl CBuffer {
    fn syncData(&self) {
        let manager = self.manager.clone();
        let serviceItems = self.serviceItems.clone();
        thread::spawn(move || {
            loop {
                CBuffer::sync(manager.clone(), serviceItems.clone());
                thread::sleep(time::Duration::from_secs(10));
            }
        });
    }

    fn sync(manager: Arc<Mutex<register::manager::CManager>>, serviceItems: Arc<Mutex<HashMap<String, service::CService>>>) {
        let mut names = Vec::new();
        {
            // avoid occupy mutex
            let items = match serviceItems.lock() {
                Ok(items) => items,
                Err(err) => {
                    println!("lock serviceItemss error, err: {}", err);
                    return;
                }
            };
            for (k, v) in items.iter() {
                // update service memory
                let item = CServiceItem{
                    name: k.clone(),
                    regCenterType: v.getRegCenterType().to_string(),
                    services: v.getServices()
                };
                names.push(item);
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
        if item.regCenterType == consts::proto::register_center_type_consul {
            let consul = match &manager.consul {
                Some(c) => c,
                None => {
                    println!("consul object is not exist");
                    return;
                }
            };
            consul.updateServices(&item.name, &item.services);
        } else {
        }
    }

    fn getServicesFromRegisterCenter(manager: Arc<Mutex<register::manager::CManager>>, name: &str, regCenterType: &str) -> Option<Vec<structs::service::CServiceInfo>> {
        let manager = match manager.lock() {
            Ok(m) => m,
            Err(err) => {
                println!("lock manager error, err: {}", err);
                return None;
            }
        };
        if regCenterType == consts::proto::register_center_type_consul {
            let consul = match &manager.consul {
                Some(c) => c,
                None => {
                    println!("consul object is not exist");
                    return None;
                }
            };
            consul.getServices(name)
        } else {
            None
        }
    }
}

impl CBuffer {
    pub fn new(centers: &Vec<structs::config::CRegisterCenter>) -> CBuffer {
        CBuffer{
            manager: Arc::new(Mutex::new(register::manager::CManager::new(centers))),
            serviceItems: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

